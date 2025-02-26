use std::io::Write;

use jj_cli::command_error::CommandError;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::{Color, Style};

/// Prints the amount of changes in the working copy
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Metrics {
    /// Controls how the changes are rendered, use {added}, {removed} and {changed} to render the number of changes.
    template: String,

    // added_files: Style,
    // removed_files: Style,
    /// Controlls how the number of changed files is rendered.
    changed_files: Metric,

    /// Controlls how the number of added lines is rendered.
    added_lines: Metric,
    /// Controlls how the number of removed lines is rendered.
    removed_lines: Metric,

    #[serde(flatten)]
    style: Style,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            style: Style {
                color: Some(Color::Magenta),
                ..Default::default()
            },
            template: "[{changed} {added}{removed}]".to_string(),
            changed_files: Metric {
                style: Style {
                    color: Some(Color::Cyan),
                    ..Default::default()
                },
                ..Default::default()
            },
            added_lines: Metric {
                style: Style {
                    color: Some(Color::Green),
                    ..Default::default()
                },
                prefix: "+".to_string(),
                ..Default::default()
            },
            removed_lines: Metric {
                style: Style {
                    color: Some(Color::Red),
                    ..Default::default()
                },
                prefix: "-".to_string(),
                ..Default::default()
            },
        }
    }
}

#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug, Default)]
struct Metric {
    #[serde(default)]
    prefix: String,
    #[serde(default)]
    suffix: String,
    #[serde(flatten)]
    style: Style,
}
impl Metric {
    fn format(&self, number: usize, global_style: &Style) -> String {
        format!(
            "{}{}{}{}{}",
            self.style.format(),
            self.prefix,
            number,
            self.suffix,
            global_style.format(),
        )
    }
}

#[derive(Debug, Serialize)]
struct Context {
    added: String,
    removed: String,
    changed: String,
}

impl Metrics {
    pub fn print(
        &self,
        io: &mut impl Write,
        data: &crate::JJData,
        module_separator: &str,
    ) -> Result<(), CommandError> {
        let context = Context {
            added: self
                .added_lines
                .format(data.commit.diff.lines_added, &self.style),
            removed: self
                .removed_lines
                .format(data.commit.diff.lines_removed, &self.style),
            changed: self
                .changed_files
                .format(data.commit.diff.files_changed, &self.style),
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

        self.style.print(io)?;

        write!(io, "{}{module_separator}", s)?;

        Ok(())
    }
}
