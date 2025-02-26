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
    module_separator: String,
    #[serde(default)]
    pub bookmarks: BookmarkConfig,
    #[serde(rename = "module")]
    modules: Vec<ModuleConfig>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BookmarkConfig {
    pub search_depth: Option<usize>,
    #[serde(default)]
    pub exclude: Vec<Glob>,
}

impl Config {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        for module in self.modules.iter() {
            match module {
                ModuleConfig::Bookmarks(bookmarks) => {
                    bookmarks.print(io, data, &self.module_separator)?;
                }
                ModuleConfig::Commit(commit_desc) => {
                    commit_desc.print(io, data, &self.module_separator)?
                }
                ModuleConfig::State(commit_warnings) => {
                    commit_warnings.print(io, data, &self.module_separator)?
                }
                ModuleConfig::Metrics(commit_diff) => {
                    commit_diff.print(io, data, &self.module_separator)?
                }
            }
        }
        util::Style::default().print(io)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
enum ModuleConfig {
    Bookmarks(Bookmarks),
    Commit(Commit),
    State(State),
    Metrics(Metrics),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            module_separator: " ".to_string(),
            modules: vec![
                ModuleConfig::Bookmarks(Default::default()),
                ModuleConfig::Commit(Default::default()),
                ModuleConfig::State(Default::default()),
                ModuleConfig::Metrics(Default::default()),
            ],
            bookmarks: Default::default(),
        }
    }
}
