use std::io::Write;

use jj_cli::command_error::CommandError;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::Style;

/// Prints the working copies commit text
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Commit {
    /// Maximum length the commit text will be truncated to.
    max_length: Option<usize>,
    /// Controls how the commit text is rendered.
    #[serde(flatten)]
    style: Style,
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            style: Default::default(),
            max_length: Some(24),
        }
    }
}

impl Commit {
    pub fn print(
        &self,
        io: &mut impl Write,
        data: &crate::JJData,
        module_separator: &str,
    ) -> Result<(), CommandError> {
        let first_line = data
            .commit
            .desc
            .split_once(['\r', '\n'])
            .map(|(line, _rest)| line)
            .unwrap_or(&data.commit.desc);

        if !first_line.is_empty() {
            self.style.print(io)?;

            match self.max_length {
                Some(max_len) if first_line.len() > max_len => {
                    write!(io, "\"{}…\"{module_separator}", &first_line[..max_len - 1])?;
                }
                _ => {
                    write!(io, "\"{}\"{module_separator}", first_line)?;
                }
            }
        }
        Ok(())
    }
}
