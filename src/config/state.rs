use std::io::Write;

use jj_cli::{cli_util::RevisionArg, command_error::CommandError, ui::Ui};
use jj_lib::repo::Repo;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::Style;

/// Prints a warning if the working copy contains any conflicts, is divergent or hidden
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    /// Text that will be printed between each Warning.
    #[serde(default = "default_separator")]
    separator: String,
    /// Controls how the conflict warning will be rendered.
    #[serde(default = "default_conflict")]
    conflict: Status,
    /// Controls how the divergence warning will be rendered.
    #[serde(default = "default_divergent")]
    divergent: Status,
    /// Controls how the divergence warning will be rendered.
    #[serde(default = "default_empty")]
    empty: Status,
    /// Controls how the empty warning will be rendered.
    #[serde(default = "default_immutable")]
    immutable: Status,
    /// Controls how the immutable warning will be rendered.
    #[serde(default = "default_hidden")]
    hidden: Status,
}

fn default_separator() -> String {
    " ".to_string()
}

fn default_conflict() -> Status {
    Status {
        text: "(CONFLICT)".to_string(),
        style: Style {
            color: Some(super::util::Color::Red),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn default_immutable() -> Status {
    Status {
        text: "(IMMUTABLE)".to_string(),
        style: Style {
            color: Some(super::util::Color::Yellow),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn default_empty() -> Status {
    Status {
        text: "(EMPTY)".to_string(),
        style: Style {
            color: Some(super::util::Color::Yellow),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn default_hidden() -> Status {
    Status {
        text: "(HIDDEN)".to_string(),
        style: Style {
            color: Some(super::util::Color::Yellow),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn default_divergent() -> Status {
    Status {
        text: "(DIVERGENT)".to_string(),
        style: Style {
            color: Some(super::util::Color::Cyan),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug, Default)]
struct Status {
    #[serde(default)]
    /// Do not render this warning
    disabled: bool,
    /// The text that should be printed when the working copy has the given state.
    text: String,
    #[serde(flatten, default)]
    style: Style,
}

impl Default for State {
    fn default() -> Self {
        Self {
            separator: default_separator(),
            conflict: default_conflict(),
            divergent: default_divergent(),
            hidden: default_hidden(),
            empty: default_empty(),
            immutable: default_immutable(),
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
        if let Some(true) = data.commit.warnings.conflict {
            self.conflict.style.print(io)?;
            first = false;
            write!(io, "{}", self.conflict.text)?;
        }
        if let Some(true) = data.commit.warnings.divergent {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.divergent.style.print(io)?;
            write!(io, "{}", self.divergent.text)?;
        }
        if let Some(true) = data.commit.warnings.hidden {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.hidden.style.print(io)?;
            write!(io, "{}", self.hidden.text)?;
        }
        if let Some(true) = data.commit.warnings.immutable {
            if !first {
                write!(io, "{}", self.separator)?;
            }
            first = false;
            self.immutable.style.print(io)?;
            write!(io, "{}", self.immutable.text)?;
        }
        if let Some(true) = data.commit.warnings.empty {
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
    pub fn parse(
        &self,
        command_helper: &jj_cli::cli_util::CommandHelper,
        state: &mut crate::State,
        data: &mut crate::JJData,
        global: &super::GlobalConfig,
    ) -> Result<(), CommandError> {
        let workspace_helper = command_helper.workspace_helper(&Ui::null())?;

        if !self.empty.disabled && data.commit.warnings.empty.is_none() {
            data.commit.warnings.empty = state.commit_is_empty(command_helper)?;
        }
        if !self.conflict.disabled && data.commit.warnings.conflict.is_none() {
            data.commit.warnings.conflict = state
                .commit(command_helper)?
                .as_ref()
                .map(|c| c.has_conflict())
                .transpose()?;
        }

        self.parse_hidden_and_divergent(command_helper, state, data, global)?;

        if !self.immutable.disabled && data.commit.warnings.immutable.is_none() {
            if let Some(commit_id) = state.commit_id(command_helper)? {
                let revs = workspace_helper
                    .parse_revset(&Ui::null(), &RevisionArg::from("immutable()".to_string()))?;

                let mut immutable = revs.evaluate_to_commit_ids()?;

                data.commit.warnings.immutable =
                    Some(immutable.any(|id| id.as_ref().is_ok_and(|id| id == commit_id)));
            }
        }

        Ok(())
    }
    pub fn parse_hidden_and_divergent(
        &self,
        command_helper: &jj_cli::cli_util::CommandHelper,
        state: &mut crate::State,
        data: &mut crate::JJData,
        _global: &super::GlobalConfig,
    ) -> Result<(), CommandError> {
        if (!self.hidden.disabled && data.commit.warnings.hidden.is_none())
            || (!self.divergent.disabled && data.commit.warnings.divergent.is_none())
        {
            let repo = state.repo(command_helper)?;
            let Some(commit) = state.commit(command_helper)? else {
                return Ok(());
            };
            let change_id = commit.change_id();
            let change = repo.resolve_change_id(change_id);

            match change {
                Some(commits) => match commits.len() {
                    0 => data.commit.warnings.hidden = Some(true),
                    1 => {}
                    _ => data.commit.warnings.divergent = Some(true),
                },
                None => data.commit.warnings.hidden = Some(true),
            }
        }
        Ok(())
    }
}
