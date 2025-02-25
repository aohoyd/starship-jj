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

        for b in data.bookmarks.iter() {
            write!(io, "{} ", b)?;
        }
        if data.bookmark_behind != 0 {
            match self.behind_symbol {
                Some(s) => write!(io, "{s}{} ", data.bookmark_behind)?,
                None => write!(io, "{} ", data.bookmark_behind)?,
            }
        }
        Ok(())
    }
}
