use std::{collections::BTreeMap, io::Write, path::PathBuf, process::ExitCode, sync::Arc};

use args::{ConfigCommands, CustomCommand, StarshipCommands};
use config::BookmarkConfig;
use jj_cli::{
    cli_util::{CliRunner, CommandHelper},
    command_error::{CommandError, user_error, user_error_with_message},
    ui::Ui,
};
use jj_lib::{backend::CommitId, store::Store, view::View};

pub use state::State;

mod args;
mod config;
mod state;

fn starship(
    ui: &mut Ui,
    command_helper: &CommandHelper,
    command: CustomCommand,
) -> Result<(), CommandError> {
    #[cfg(feature = "json-schema")]
    {
        let schema = schemars::schema_for!(config::Config);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        return Ok(());
    }

    let CustomCommand::Starship(args) = command;
    match args.command {
        StarshipCommands::Prompt { starship_config } => {
            print_prompt(command_helper, &starship_config)?
        }
        StarshipCommands::Config(ConfigCommands::Path) => {
            let config_dir = get_config_path()?;

            writeln!(ui.stdout(), "{}", config_dir)?;
        }
        StarshipCommands::Config(ConfigCommands::Default) => {
            let c = toml::to_string_pretty(&config::Config::default()).map_err(user_error)?;

            writeln!(ui.stdout(), "{}", c)?;
        }
    }

    Ok(())
}

fn get_config_path() -> Result<String, CommandError> {
    let config_dir = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|p| p.join(".config")))
        .ok_or_else(|| user_error("Failed to find config dir"))?;
    let config_dir = config_dir.join("starship-jj/starship-jj.toml");
    let config_dir = config_dir
        .to_str()
        .ok_or_else(|| user_error("The config path is not valid UTF-8"))?;
    Ok(config_dir.to_string())
}

#[derive(Default)]
struct JJData {
    bookmarks: Option<BTreeMap<String, usize>>,
    commit: CommitData,
}

#[derive(Default)]
struct CommitData {
    desc: Option<String>,
    warnings: CommitWarnings,
    diff: Option<CommitDiff>,
}

#[derive(Default)]
struct CommitWarnings {
    hidden: Option<bool>,
    conflict: Option<bool>,
    divergent: Option<bool>,
    immutable: Option<bool>,
    empty: Option<bool>,
}

#[derive(Default)]
struct CommitDiff {
    // files_added : usize,
    // files_removed : usize,
    files_changed: usize,
    lines_added: usize,
    lines_removed: usize,
}

fn print_prompt(
    command_helper: &CommandHelper,
    config_path: &Option<PathBuf>,
) -> Result<(), CommandError> {
    let config = if let Some(config_path) = config_path {
        toml::from_str(
            &std::fs::read_to_string(config_path)
                .map_err(|e| user_error_with_message("Failed to read Config File", e))?,
        )
        .map_err(|e| user_error_with_message("Failed to read Config File", e))?
    } else {
        let config_dir = get_config_path()?;
        if std::fs::exists(&config_dir)? {
            toml::from_str(&std::fs::read_to_string(config_dir)?).map_err(user_error)?
        } else {
            config::Config::default()
        }
    };

    let mut state = State::default();
    let mut data = JJData::default();

    config.print(&command_helper, &mut state, &mut data)?;

    Ok(())
}

fn find_parent_bookmarks(
    commit_id: &CommitId,
    depth: usize,
    config: &BookmarkConfig,
    bookmarks: &mut BTreeMap<String, usize>,
    view: &View,
    store: &Arc<Store>,
) -> Result<(), CommandError> {
    let tmp: Vec<_> = view
        .local_bookmarks_for_commit(commit_id)
        .map(|(name, _)| name)
        .collect();

    if !tmp.is_empty() {
        'bookmark: for bookmark in tmp {
            for glob in &config.exclude {
                #[cfg(not(feature = "json-schema"))]
                if glob.matches(bookmark) {
                    continue 'bookmark;
                }
            }
            let bookmark = bookmark.to_string();
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
    let start = std::time::Instant::now();
    let print_timing = std::env::var("STARSHIP_JJ_TIMING").is_ok();
    let clirunner = CliRunner::init();
    let clirunner = clirunner.add_subcommand(starship);
    let e = clirunner.run();
    let elapsed = start.elapsed();
    if print_timing {
        print!("{elapsed:?} ");
    }
    e
}
