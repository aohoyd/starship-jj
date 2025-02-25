use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::Style;

#[derive(Deserialize, Serialize, Debug)]
pub struct CommitWarnings {
    conflict: Style,
    divergent: Style,
    hidden: Style,
}

impl Default for CommitWarnings {
    fn default() -> Self {
        Self {
            conflict: Style {
                color: Some(super::util::Color::Red),
                ..Default::default()
            },
            divergent: Style {
                color: Some(super::util::Color::Cyan),
                ..Default::default()
            },
            hidden: Style {
                color: Some(super::util::Color::Yellow),
                ..Default::default()
            },
        }
    }
}

impl CommitWarnings {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        if data.commit_is_conflict {
            self.conflict.print(io)?;
            write!(io, "(CONFLICT) ")?;
        }
        if data.commit_is_divergent {
            self.divergent.print(io)?;
            write!(io, "(DIVERGENT) ")?;
        }
        if data.commit_is_hidden {
            self.hidden.print(io)?;
            write!(io, "(HIDDEN) ")?;
        }
        Ok(())
    }
}
