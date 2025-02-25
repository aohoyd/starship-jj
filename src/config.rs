use std::io::Write;

use bookmarks::Bookmarks;
use commit::Commit;
use jj_cli::command_error::CommandError;
use metrics::Metrics;
use serde::{Deserialize, Serialize};
use state::State;
use util::Glob;

pub mod util;

mod bookmarks;
mod commit;
mod metrics;
mod state;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    modules: Vec<ModuleConfig>,
    pub bookmark_search_depth: Option<usize>,
    pub excluded_bookmarks: Vec<Glob>,
}

impl Config {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        for module in self.modules.iter() {
            match module {
                ModuleConfig::Bookmarks(bookmarks) => {
                    bookmarks.print(io, &data)?;
                }
                ModuleConfig::Commit(commit_desc) => commit_desc.print(io, data)?,
                ModuleConfig::State(commit_warnings) => commit_warnings.print(io, data)?,
                ModuleConfig::Metrics(commit_diff) => commit_diff.print(io, data)?,
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum ModuleConfig {
    Bookmarks(Bookmarks),
    Commit(Commit),
    State(State),
    Metrics(Metrics),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![
                ModuleConfig::Bookmarks(Default::default()),
                ModuleConfig::Commit(Default::default()),
                ModuleConfig::State(Default::default()),
                ModuleConfig::Metrics(Default::default()),
            ],
            bookmark_search_depth: None,
            excluded_bookmarks: Default::default(),
        }
    }
}
