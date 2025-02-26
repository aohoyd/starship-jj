use glob::Pattern;
use jj_cli::command_error::CommandError;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(try_from = "&str", into = "String")]
pub struct Glob(glob::Pattern);
impl TryFrom<&str> for Glob {
    type Error = glob::PatternError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Pattern::new(value)?))
    }
}
impl From<Glob> for String {
    fn from(value: Glob) -> Self {
        value.0.as_str().to_string()
    }
}

impl Glob {
    pub fn matches(&self, haystack: &str) -> bool {
        self.0.matches(haystack)
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Style {
    pub color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Style {
    pub fn print(&self, io: &mut impl Write) -> Result<(), CommandError> {
        write!(io, "\x1B[")?;

        if let Some(color) = self.color {
            write!(io, "{}", colored::Color::from(color).to_fg_str())?;
        } else {
            write!(io, "39")?;
        }
        if let Some(color) = self.bg_color {
            write!(io, ";{}", colored::Color::from(color).to_bg_str())?;
        } else {
            write!(io, ";49")?;
        }

        write!(io, "m")?;
        Ok(())
    }

    pub fn format(&self) -> String {
        let mut s = "\x1B[".to_string();

        if let Some(color) = self.color {
            s.push_str(colored::Color::from(color).to_fg_str().as_ref());
        } else {
            s.push_str("39");
        }
        s.push(';');
        if let Some(color) = self.bg_color {
            s.push_str(colored::Color::from(color).to_bg_str().as_ref());
        } else {
            s.push_str("49");
        }

        s.push('m');
        s
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

impl From<Color> for colored::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => colored::Color::Black,
            Color::Red => colored::Color::Red,
            Color::Green => colored::Color::Green,
            Color::Yellow => colored::Color::Yellow,
            Color::Blue => colored::Color::Blue,
            Color::Magenta => colored::Color::Magenta,
            Color::Cyan => colored::Color::Cyan,
            Color::White => colored::Color::White,
            Color::BrightBlack => colored::Color::BrightBlack,
            Color::BrightRed => colored::Color::BrightRed,
            Color::BrightGreen => colored::Color::BrightGreen,
            Color::BrightYellow => colored::Color::BrightYellow,
            Color::BrightBlue => colored::Color::BrightBlue,
            Color::BrightMagenta => colored::Color::BrightMagenta,
            Color::BrightCyan => colored::Color::BrightCyan,
            Color::BrightWhite => colored::Color::BrightWhite,
            Color::TrueColor { r, g, b } => colored::Color::TrueColor { r, g, b },
        }
    }
}

impl From<colored::Color> for Color {
    fn from(value: colored::Color) -> Self {
        match value {
            colored::Color::Black => Color::Black,
            colored::Color::Red => Color::Red,
            colored::Color::Green => Color::Green,
            colored::Color::Yellow => Color::Yellow,
            colored::Color::Blue => Color::Blue,
            colored::Color::Magenta => Color::Magenta,
            colored::Color::Cyan => Color::Cyan,
            colored::Color::White => Color::White,
            colored::Color::BrightBlack => Color::BrightBlack,
            colored::Color::BrightRed => Color::BrightRed,
            colored::Color::BrightGreen => Color::BrightGreen,
            colored::Color::BrightYellow => Color::BrightYellow,
            colored::Color::BrightBlue => Color::BrightBlue,
            colored::Color::BrightMagenta => Color::BrightMagenta,
            colored::Color::BrightCyan => Color::BrightCyan,
            colored::Color::BrightWhite => Color::BrightWhite,
            colored::Color::TrueColor { r, g, b } => Color::TrueColor { r, g, b },
        }
    }
}
