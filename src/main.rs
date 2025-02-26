use std::{collections::BTreeMap, io::Write, process::ExitCode, sync::Arc};

use args::{ConfigCommands, CustomCommand, StarshipCommands};
use config::BookmarkConfig;
use jj_cli::{
    cli_util::{CliRunner, CommandHelper},
    command_error::{user_error, CommandError},
    diff_util::{get_copy_records, DiffStatOptions, DiffStats},
    ui::Ui,
};
use jj_lib::{backend::CommitId, copies::CopyRecords, repo::Repo, store::Store, view::View};
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
        StarshipCommands::Config(ConfigCommands::Path) => {
            let config_dir = dirs::config_dir()
                .or_else(|| dirs::home_dir().map(|p| p.join(".config")))
                .ok_or_else(|| user_error("Failed to find config dir"))?;
            let config_dir = config_dir.join("starship-jj/starship-jj.toml");

            let config_dir = config_dir
                .to_str()
                .ok_or_else(|| user_error("The config path is not valid UTF-8"))?;

            writeln!(ui.stdout(), "{}", config_dir)?;
        }
        StarshipCommands::Config(ConfigCommands::Default) => {
            let c = toml::to_string_pretty(&config::Config::default()).map_err(user_error)?;

            writeln!(ui.stdout(), "{}", c)?;
        }
    }

    Ok(())
}

#[derive(Default)]
struct JJData<'a> {
    bookmarks: BTreeMap<&'a str, usize>,
    commit: CommitData,
}

#[derive(Default)]
struct CommitData {
    desc: String,
    warnings: CommitWarnings,
    diff: CommitDiff,
}

#[derive(Default)]
struct CommitWarnings {
    hidden: bool,
    conflict: bool,
    divergent: bool,
}

#[derive(Default)]
struct CommitDiff {
    // files_added : usize,
    // files_removed : usize,
    files_changed: usize,
    lines_added: usize,
    lines_removed: usize,
}

fn print_prompt(ui: &mut Ui, command_helper: &CommandHelper) -> Result<(), CommandError> {
    let config = config::Config::default();
    let workspace_helper = command_helper.workspace_helper(ui)?;
    let repo = workspace_helper.repo();
    let store = repo.store();
    let mut data = JJData::default();

    let Some(commit_id) = workspace_helper.get_wc_commit_id() else {
        return Ok(());
    };

    let commit = store.get_commit(commit_id)?;

    let matcher = workspace_helper.parse_file_patterns(ui, &[])?.to_matcher();
    let change_id = commit.change_id();
    let change = repo.resolve_change_id(change_id);
    let mut copy_records = CopyRecords::default();
    for parent in commit.parent_ids() {
        let records = get_copy_records(repo.store(), parent, commit_id, &matcher)?;
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

    data.commit.diff.files_changed = stats.entries().len();
    data.commit.diff.lines_added = stats.count_total_added();
    data.commit.diff.lines_removed = stats.count_total_removed();

    data.commit.desc = commit.description().to_string();
    data.commit.warnings.conflict = commit.has_conflict()?;

    match change {
        Some(commits) => match commits.len() {
            0 => data.commit.warnings.hidden = true,
            1 => {}
            _ => data.commit.warnings.divergent = true,
        },
        None => data.commit.warnings.hidden = true,
    }

    find_parent_bookmarks(
        commit_id,
        0,
        &config.bookmarks,
        &mut data.bookmarks,
        repo.view(),
        store,
    )?;

    let mut io = ui.stdout();

    config.print(&mut io, &data)?;

    Ok(())
}

fn find_parent_bookmarks<'a>(
    commit_id: &CommitId,
    depth: usize,
    config: &BookmarkConfig,
    bookmarks: &mut BTreeMap<&'a str, usize>,
    view: &'a View,
    store: &Arc<Store>,
) -> Result<(), CommandError> {
    let tmp: Vec<_> = view
        .local_bookmarks_for_commit(commit_id)
        .map(|(name, _)| name)
        .collect();

    if !tmp.is_empty() {
        'bookmark: for bookmark in tmp {
            for glob in &config.exclude {
                if glob.matches(bookmark) {
                    continue 'bookmark;
                }
            }
            bookmarks
                .entry(bookmark)
                .and_modify(|v| {
                    if *v > depth {
                        *v = depth
                    }
                })
                .or_insert(depth);
        }
        return Ok(());
    }

    if let Some(max_depth) = config.search_depth {
        if depth >= max_depth {
            return Ok(());
        }
    }

    let commit = store.get_commit(commit_id)?;

    for p in commit.parent_ids() {
        find_parent_bookmarks(p, depth + 1, config, bookmarks, view, store)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    let clirunner = CliRunner::init();
    let clirunner = clirunner.add_subcommand(starship);
    clirunner.run()
}
