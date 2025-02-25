use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::Style;

#[derive(Deserialize, Serialize, Debug)]
pub struct CommitDesc {
    #[serde(flatten)]
    style: Style,
    max_length: Option<usize>,
}

impl Default for CommitDesc {
    fn default() -> Self {
        Self {
            style: Default::default(),
            max_length: Some(24),
        }
    }
}

impl CommitDesc {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        let first_line = data
            .commit_desc
            .split_once(|c| c == '\r' || c == '\n')
            .map(|(line, _rest)| line)
            .unwrap_or(&data.commit_desc);

        if !first_line.is_empty() {
            self.style.print(io)?;

            match self.max_length {
                Some(max_len) if first_line.len() > max_len => {
                    write!(io, "\"{}…\" ", &first_line[..max_len - 1])?;
                }
                _ => {
                    write!(io, "\"{}\" ", first_line)?;
                }
            }
        }
        Ok(())
    }
}
