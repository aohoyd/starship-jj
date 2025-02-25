use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::Style;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CommitWarnings {
    conflict: Style,
    divergent: Style,
    hidden: Style,
}

impl CommitWarnings {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        if data.commit.warnings.conflict {
            self.conflict.print(io)?;
            write!(io, "(CONFLICT) ")?;
        }
        if data.commit.warnings.divergent {
            self.divergent.print(io)?;
            write!(io, "(DIVERGENT) ")?;
        }
        if data.commit.warnings.hidden {
            self.hidden.print(io)?;
            write!(io, "(HIDDEN) ")?;
        }
        Ok(())
    }
}
