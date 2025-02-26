use std::io::Write;

use bookmarks::Bookmarks;
use commit::Commit;
use jj_cli::command_error::CommandError;
use metrics::Metrics;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use state::State;
use util::Glob;

pub mod util;

mod bookmarks;
mod commit;
mod metrics;
mod state;

#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    /// Text that will be printed between each Module.
    module_separator: String,
    /// Controls the behaviour of the bookmark finding algorythm.
    #[serde(default)]
    pub bookmarks: BookmarkConfig,
    /// Modules that will be rendered.
    #[serde(rename = "module")]
    modules: Vec<ModuleConfig>,
}

#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BookmarkConfig {
    /// Controls how far we are looking back to find bookmarks.
    pub search_depth: Option<usize>,
    /// Exclude certain bookmarks from the search (supports globs)
    #[serde(default)]
    #[cfg(feature = "json-schema")]
    pub exclude: Vec<String>,
    #[cfg(not(feature = "json-schema"))]
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

/// A module that prints some info about the current jj repo
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
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
