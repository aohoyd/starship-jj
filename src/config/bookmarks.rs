use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::{Color, Style};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bookmarks {
    #[serde(flatten)]
    style: Style,
    behind_symbol: Option<char>,
    number: Option<usize>,
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self {
            style: Style {
                color: Some(Color::Magenta),
                ..Default::default()
            },
            behind_symbol: Some('⇡'),
            number: None,
        }
    }
}

impl Bookmarks {
    pub fn print(&self, io: &mut impl Write, data: &crate::JJData) -> Result<(), CommandError> {
        self.style.print(io)?;

        let mut ordered: BTreeMap<usize, BTreeSet<&str>> = BTreeMap::new();

        for (name, behind) in &data.bookmarks {
            ordered
                .entry(*behind)
                .and_modify(|s| {
                    s.insert(*name);
                })
                .or_insert_with(|| {
                    let mut s = BTreeSet::new();
                    s.insert(*name);
                    s
                });
        }

        let mut counter = 0;
        'outer: for (behind, bookmarks) in ordered {
            for name in bookmarks {
                if let Some(number) = self.number {
                    if counter >= number {
                        write!(io, "…")?;
                        break 'outer;
                    }
                }
                if counter > 0 {
                    write!(io, " ")?;
                }
                write!(io, "{}", name)?;
                if behind != 0 {
                    match self.behind_symbol {
                        Some(s) => write!(io, "{s}{}", behind)?,
                        None => write!(io, "{}", behind)?,
                    }
                }
                counter += 1;
            }
        }

        write!(io, " ")?;
        Ok(())
    }
}
