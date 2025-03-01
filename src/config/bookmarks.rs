use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

use jj_cli::{command_error::CommandError, ui::Ui};
use jj_lib::repo::Repo;
#[cfg(feature = "json-schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::util::{Color, Style};

/// Prints information about bookmarks in the working copies ancestors.
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Debug)]
pub struct Bookmarks {
    /// Text that will be rendered between each bookmark.
    separator: String,
    /// Controls how bookmarks are rendered.
    #[serde(flatten)]
    style: Style,
    /// A suffix that will be printed when the given bookmark is behing the working copy.
    behind_symbol: Option<char>,
    /// Maximum amout of bookmarks that will be rendered.
    max_bookmarks: Option<usize>,
    /// Maximum length the bookmark name will be truncated to.
    max_length: Option<usize>,
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
            max_length: None,
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
        let Some(bookmarks) = data.bookmarks.as_ref() else {
            unreachable!()
        };

        self.style.print(io)?;

        let mut ordered: BTreeMap<usize, BTreeSet<&String>> = BTreeMap::new();

        for (name, behind) in bookmarks {
            ordered
                .entry(*behind)
                .and_modify(|s| {
                    s.insert(name);
                })
                .or_insert_with(|| {
                    let mut s = BTreeSet::new();
                    s.insert(name);
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
                match self.max_length {
                    Some(max_len) if name.len() > max_len => {
                        write!(io, "\"{}…\"", &name[..max_len - 1])?;
                    }
                    _ => {
                        write!(io, "\"{}\"", name)?;
                    }
                }
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

    pub(crate) fn parse(
        &self,
        ui: &mut Ui,
        command_helper: &jj_cli::cli_util::CommandHelper,
        state: &mut crate::State,
        data: &mut crate::JJData,
        global: &super::GlobalConfig,
    ) -> Result<(), CommandError> {
        if data.bookmarks.is_some() {
            return Ok(());
        }
        let mut bookmarks = BTreeMap::new();

        let repo = state.repo(command_helper, ui)?;
        let view = repo.view();
        let store = repo.store();
        let Some(commit_id) = state.commit_id(command_helper, ui)? else {
            return Ok(());
        };

        crate::find_parent_bookmarks(commit_id, 0, &global.bookmarks, &mut bookmarks, view, store)?;

        data.bookmarks = Some(bookmarks);
        Ok(())
    }
}
