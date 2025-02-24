use std::io::Write;

use bookmarks::Bookmarks;
use commit_desc::CommitDesc;
use commit_warnings::CommitWarnings;
use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

mod util;

mod bookmarks;
mod commit_desc;
mod commit_warnings;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    modules: Vec<ModuleConfig>,
}

impl Config {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        for module in self.modules.iter() {
            match module {
                ModuleConfig::Bookmarks(bookmarks) => {
                    bookmarks.print(io, &data)?;
                }
                ModuleConfig::CommitDesc(commit_desc) => commit_desc.print(io, data)?,
                ModuleConfig::CommitWarnings(commit_warnings) => commit_warnings.print(io, data)?,
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum ModuleConfig {
    Bookmarks(Bookmarks),
    CommitDesc(CommitDesc),
    CommitWarnings(CommitWarnings),
    // CommitState(CommitState),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![
                ModuleConfig::Bookmarks(Default::default()),
                ModuleConfig::CommitDesc(Default::default()),
            ],
        }
    }
}
