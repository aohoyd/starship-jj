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
    #[serde(default = "default_max_length")]
    max_length: Option<usize>,
    /// Controls how the commit text is rendered.
    #[serde(flatten)]
    style: Style,
}

fn default_max_length() -> Option<usize> {
    Some(24)
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            style: Default::default(),
            max_length: default_max_length(),
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
        let Some(desc) = data.commit.desc.as_ref() else {
            return Ok(());
        };

        let first_line = desc
            .split_once(['\r', '\n'])
            .map(|(line, _rest)| line)
            .unwrap_or(desc);

        if !first_line.is_empty() {
            self.style.print(io)?;

            crate::print_ansi_truncated(self.max_length, io, first_line)?;
            write!(io, "{module_separator}")?;
        }
        Ok(())
    }
    pub(crate) fn parse(
        &self,
        command_helper: &jj_cli::cli_util::CommandHelper,
        state: &mut crate::State,
        data: &mut crate::JJData,
        _global: &super::GlobalConfig,
    ) -> Result<(), CommandError> {
        if data.commit.desc.is_some() {
            return Ok(());
        }
        let Some(commit) = state.commit(command_helper)? else {
            return Ok(());
        };
        data.commit.desc = Some(commit.description().to_string());
        Ok(())
    }
}
