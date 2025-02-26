use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};

use super::util::{Color, Style};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bookmarks {
    separator: String,
    #[serde(flatten)]
    style: Style,
    behind_symbol: Option<char>,
    max_bookmarks: Option<usize>,
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self {
            style: Style {
                color: Some(Color::Magenta),
                ..Default::default()
            },
            behind_symbol: Some('⇡'),
            max_bookmarks: None,
            separator: " ".to_string(),
        }
    }
}

impl Bookmarks {
    pub fn print(
        &self,
        io: &mut impl Write,
        data: &crate::JJData,
        module_separator: &str,
    ) -> Result<(), CommandError> {
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
                if let Some(number) = self.max_bookmarks {
                    if counter >= number {
                        write!(io, "…{module_separator}")?;
                        // set counter to 0 so we don't print the module separator twice
                        counter = 0;
                        break 'outer;
                    }
                }
                if counter > 0 {
                    write!(io, "{}", self.separator)?;
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
        if counter != 0 {
            write!(io, "{module_separator}")?;
        }

        Ok(())
    }
}
