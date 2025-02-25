use std::process::ExitCode;

use args::{CustomCommand, StarshipCommands};
use jj_cli::{
    cli_util::{CliRunner, CommandHelper, RevisionArg},
    command_error::CommandError,
    diff_util::{get_copy_records, DiffStatOptions, DiffStats},
    ui::Ui,
};
use jj_lib::{copies::CopyRecords, graph::TopoGroupedGraphIterator, repo::Repo};
use pollster::FutureExt;

mod args;
mod config;

fn starship(
    ui: &mut Ui,
    command_helper: &CommandHelper,
    command: CustomCommand,
) -> Result<(), CommandError> {
    let CustomCommand::Starship(args) = command;
    match args.command {
        StarshipCommands::Prompt => print_prompt(ui, command_helper)?,
        StarshipCommands::Config(_) => todo!(),
    }

    Ok(())
}

#[derive(Default)]
struct JJData<'a> {
    bookmarks: Vec<&'a str>,
    bookmark_behind: usize,
    commit_desc: String,
    commit_is_hidden: bool,
    commit_is_conflict: bool,
    commit_is_divergent: bool,
    // commit_files_added : usize,
    // commit_files_removed : usize,
    commit_files_changed: usize,
    commit_lines_added: usize,
    commit_lines_removed: usize,
}

fn print_prompt(ui: &mut Ui, command_helper: &CommandHelper) -> Result<(), CommandError> {
    let config = config::Config::default();
    let workspace_helper = command_helper.workspace_helper(ui)?;
    let repo = workspace_helper.repo();
    let store = repo.store();
    let revset_eval = workspace_helper.parse_revset(ui, &RevisionArg::from("..@".to_string()))?;
    let view = repo.view();
    let revset = revset_eval.evaluate()?;
    let forward_iter = TopoGroupedGraphIterator::new(revset.iter_graph());

    let mut data = JJData::default();

    for (index, commit) in forward_iter.enumerate() {
        let Ok((commit_id, _edges)) = commit else {
            continue;
        };

        if index == 0 {
            let matcher = workspace_helper.parse_file_patterns(ui, &[])?.to_matcher();
            let commit = store.get_commit(&commit_id)?;
            let change_id = commit.change_id();
            let change = repo.resolve_change_id(&change_id);
            let mut copy_records = CopyRecords::default();
            for parent in commit.parent_ids() {
                let records = get_copy_records(repo.store(), parent, &commit_id, &matcher)?;
                copy_records.add_records(records)?;
            }

            let tree = commit.tree()?;
            let parent_tree = commit.parent_tree(repo.as_ref())?;

            let tree_diff = parent_tree.diff_stream_with_copies(&tree, &matcher, &copy_records);
            let stats = DiffStats::calculate(
                store,
                tree_diff,
                &DiffStatOptions::default(),
                jj_lib::conflicts::ConflictMarkerStyle::Diff,
            )
            .block_on()?;

            data.commit_files_changed = stats.entries().len();
            data.commit_lines_added = stats.count_total_added();
            data.commit_lines_removed = stats.count_total_removed();

            data.commit_desc = commit.description().to_string();
            data.commit_is_conflict = commit.has_conflict()?;

            match change {
                Some(commits) => match commits.len() {
                    0 => data.commit_is_hidden = true,
                    1 => {}
                    _ => data.commit_is_divergent = true,
                },
                None => data.commit_is_hidden = true,
            }
        }

        let tmp: Vec<_> = view
            .local_bookmarks_for_commit(&commit_id)
            .map(|(name, _)| name)
            .collect();

        if !tmp.is_empty() {
            data.bookmarks = tmp;
            data.bookmark_behind = index;
            break;
        }
    }

    let mut io = ui.stdout();

    config.print(&mut io, &data)?;

    Ok(())
}

fn main() -> ExitCode {
    let clirunner = CliRunner::init();
    let clirunner = clirunner.add_subcommand(starship);
    clirunner.run()
}
