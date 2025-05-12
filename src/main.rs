use std::{collections::BTreeMap, io::Write, path::PathBuf, process::ExitCode, sync::Arc};

use ::config::Environment;
use args::{ConfigCommands, CustomCommand, StarshipCommands};
use config::BookmarkConfig;
use etcetera::BaseStrategy as _;
use jj_cli::{
    cli_util::{CliRunner, CommandHelper},
    command_error::{CommandError, user_error},
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
    let config_dir = etcetera::choose_base_strategy()
        .ok()
        .map(|s| s.config_dir())
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
    let _ = dotenvy::dotenv();
    let mut b = ::config::Config::builder();

    if let Some(config_path) = config_path {
        b = b.add_source(::config::File::new(
            config_path.to_str().ok_or(CommandError::new(
                jj_cli::command_error::CommandErrorKind::User,
                "Invalid Config Path",
            ))?,
            ::config::FileFormat::Toml,
        ));
    } else {
        let config_dir = get_config_path()?;
        if std::fs::exists(&config_dir)? {
            b = b.add_source(::config::File::new(&config_dir, ::config::FileFormat::Toml));
        } else {
            b = b.add_source(
                ::config::Config::try_from(&config::Config::default())
                    .expect("Config not serializable?"),
            );
        }
    };

    b = b.add_source(
        Environment::with_prefix("SJJ")
            .separator("__")
            .prefix_separator("__")
            .try_parsing(true),
    );

    let c = b.build().map_err(|err| {
        CommandError::with_message(
            jj_cli::command_error::CommandErrorKind::User,
            "Failed to parse Config",
            err,
        )
    })?;

    let config: config::Config = c.try_deserialize().map_err(|err| {
        CommandError::with_message(
            jj_cli::command_error::CommandErrorKind::User,
            "Failed to parse Config",
            err,
        )
    })?;

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
            let bookmark = bookmark.as_str();
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

    if depth >= config.search_depth {
        return Ok(());
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

fn print_ansi_truncated(
    max_length: Option<usize>,
    io: &mut impl Write,
    name: &str,
) -> Result<(), CommandError> {
    match max_length {
        Some(max_len) if ansi_width::ansi_width(name) > max_len => {
            let ansi_max_len = name
                .char_indices()
                .map(|(i, _)| i)
                .take_while(|i| ansi_width::ansi_width(&name[..*i]) < max_len)
                .last()
                .unwrap_or_default();

            write!(io, "\"{}…\"", &name[..ansi_max_len])?;
        }
        _ => {
            write!(io, "\"{}\"", name)?;
        }
    }
    Ok(())
}
