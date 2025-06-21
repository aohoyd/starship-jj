use std::io::Write;

use jj_cli::command_error::CommandError;
use jj_lib::repo::Repo;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::Style;

/// Prints the working copies commit text
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Change {
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

impl Default for Change {
    fn default() -> Self {
        Self {
            style: Default::default(),
            max_length: default_max_length(),
        }
    }
}

impl Change {
    pub fn print(
        &self,
        io: &mut impl Write,
        data: &crate::JJData,
        module_separator: &str,
    ) -> Result<(), CommandError> {
        let Some(change_id) = data.commit.id.as_ref() else {
            return Ok(());
        };

        if !change_id.is_empty() {
            write!(io, "{}", module_separator)?;
            self.style.print(io, None)?;
            crate::print_ansi_truncated(self.max_length, io, change_id)?;
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
        if data.commit.id.is_some() {
            return Ok(());
        }
        let repo = state.repo(command_helper)?;
        let Some(commit) = state.commit(command_helper)? else {
            return Ok(());
        };
        let id_len = repo.shortest_unique_change_id_prefix_len(commit.change_id());
        let mut id = commit.change_id().to_string();
        id.truncate(id_len);
        data.commit.id = Some(id);
        Ok(())
    }
}
