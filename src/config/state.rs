use std::io::Write;

use jj_cli::command_error::CommandError;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::Style;

/// Prints a warning if the working copy contains any conflicts, is divergent or hidden
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    /// Text that will be printed between each Warning.
    separator: String,
    /// Controls how the conflict warning will be rendered.
    conflict: Status,
    /// Controls how the divergence warning will be rendered.
    divergent: Status,
    /// Controls how the divergence warning will be rendered.
    empty: Status,
    /// Controls how the empty warning will be rendered.
    immutable: Status,
    /// Controls how the immutable warning will be rendered.
    hidden: Status,
}

#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
struct Status {
    /// The text that should be printed when the working copy has the given state.
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
            empty: Status {
                text: "(EMPTY)".to_string(),
                style: Style {
                    color: Some(super::util::Color::Yellow),
                    ..Default::default()
                },
            },
            immutable: Status {
                text: "(IMMUTABLE)".to_string(),
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
        if data.commit.warnings.immutable {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.immutable.style.print(io)?;
            write!(io, "{}", self.immutable.text)?;
        }
        if data.commit.warnings.empty {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.empty.style.print(io)?;
            write!(io, "{}", self.empty.text)?;
        }
        if !first {
            write!(io, "{module_separator}")?;
        }
        Ok(())
    }
}
