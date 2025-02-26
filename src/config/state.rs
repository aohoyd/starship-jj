use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::Style;

#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    separator: String,
    conflict: Status,
    divergent: Status,
    hidden: Status,
}

#[derive(Deserialize, Serialize, Debug)]
struct Status {
    text: String,
    #[serde(flatten)]
    style: Style,
}

impl Default for State {
    fn default() -> Self {
        Self {
            separator: " ".to_string(),
            conflict: Status {
                text: "(CONFLICT)".to_string(),
                style: Style {
                    color: Some(super::util::Color::Red),
                    ..Default::default()
                },
            },
            divergent: Status {
                text: "(DIVERGENT)".to_string(),
                style: Style {
                    color: Some(super::util::Color::Cyan),
                    ..Default::default()
                },
            },
            hidden: Status {
                text: "(HIDDEN)".to_string(),
                style: Style {
                    color: Some(super::util::Color::Yellow),
                    ..Default::default()
                },
            },
        }
    }
}

impl State {
    pub fn print(
        &self,
        io: &mut impl Write,
        data: &crate::JJData,
        module_separator: &str,
    ) -> Result<(), CommandError> {
        let mut first = true;
        if data.commit.warnings.conflict {
            self.conflict.style.print(io)?;
            first = false;
            write!(io, "{}", self.conflict.text)?;
        }
        if data.commit.warnings.divergent {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.divergent.style.print(io)?;
            write!(io, "{}", self.divergent.text)?;
        }
        if data.commit.warnings.hidden {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.hidden.style.print(io)?;
            write!(io, "{}", self.hidden.text)?;
        }
        if !first {
            write!(io, "{module_separator}")?;
        }
        Ok(())
    }
}
