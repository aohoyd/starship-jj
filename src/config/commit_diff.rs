use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::Color;

#[derive(Deserialize, Serialize, Debug)]
pub struct CommitDiff {
    #[serde(flatten)]
    style: super::util::Style,

    template: String,

    // added_files: Style,
    // removed_files: Style,
    changed_files: Style,

    added_lines: Style,
    removed_lines: Style,
}

impl Default for CommitDiff {
    fn default() -> Self {
        Self {
            style: super::util::Style {
                color: Some(Color::Magenta),
                ..Default::default()
            },
            template: "[{changed} {added}{removed}]".to_string(),
            changed_files: Style {
                style: super::util::Style {
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                ..Default::default()
            },
            added_lines: Style {
                style: super::util::Style {
                    color: Some(Color::Green),
                    ..Default::default()
                },
                prefix: "+".to_string(),
                ..Default::default()
            },
            removed_lines: Style {
                style: super::util::Style {
                    color: Some(Color::Red),
                    ..Default::default()
                },
                prefix: "-".to_string(),
                ..Default::default()
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Style {
    #[serde(flatten)]
    style: super::util::Style,
    suffix: String,
    prefix: String,
}
impl Style {
    fn format(&self, number: usize) -> String {
        format!(
            "{}{}{}{}",
            self.style.format(),
            self.prefix,
            number,
            self.suffix
        )
    }
}

#[derive(Debug, Serialize)]
struct Context {
    added: String,
    removed: String,
    changed: String,
}

impl CommitDiff {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        let context = Context {
            added: self.added_lines.format(data.commit_lines_added),
            removed: self.removed_lines.format(data.commit_lines_removed),
            changed: self.changed_files.format(data.commit_files_changed),
        };
        let mut tiny_template = tinytemplate::TinyTemplate::new();
        tiny_template
            .add_template("template", &self.template)
            .map_err(|e| {
                CommandError::with_message(
                    jj_cli::command_error::CommandErrorKind::Internal,
                    "template",
                    e,
                )
            })?;
        let s = tiny_template.render("template", &context).map_err(|e| {
            CommandError::with_message(
                jj_cli::command_error::CommandErrorKind::Internal,
                "template",
                e,
            )
        })?;

        write!(io, "{} ", s)?;

        Ok(())
    }
}
