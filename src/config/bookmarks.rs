use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::{Color, Style};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bookmarks {
    #[serde(flatten)]
    style: Style,
    behind_symbol: Option<char>,
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self {
            style: Style {
                color: Some(Color::Magenta),
                ..Default::default()
            },
            behind_symbol: Some('⇡'),
        }
    }
}

impl Bookmarks {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        self.style.print(io)?;

        for (name, behind) in &data.bookmarks {
            write!(io, "{}", name)?;
            if *behind != 0 {
                match self.behind_symbol {
                    Some(s) => write!(io, "{s}{}", behind)?,
                    None => write!(io, "{}", behind)?,
                }
            }
            write!(io, " ")?;
        }
        Ok(())
    }
}
