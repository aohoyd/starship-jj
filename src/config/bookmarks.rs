use std::io::Write;

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Bookmarks {
    #[serde(flatten)]
    style: super::util::Style,
    max_length: Option<usize>,
    behind_symbol: Option<char>,
}

impl Bookmarks {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        self.style.print(io)?;

        for (name, behind) in &data.bookmarks {
            write!(io, "{} ", name)?;
            if *behind != 0 {
                match self.behind_symbol {
                    Some(s) => write!(io, "{s}{} ", behind)?,
                    None => write!(io, "{} ", behind)?,
                }
            }
        }
        Ok(())
    }
}
