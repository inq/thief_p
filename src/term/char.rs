use term;
use util;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Char {
    pub chr: char,
    pub brush: term::Brush,
}

impl fmt::Display for Char {
    /// Write the character.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chr)
    }
}

impl Char {
    pub fn new(chr: char, brush: term::Brush) -> Char {
        Char {
            chr: chr,
            brush: brush,
        }
    }

    pub fn width(&self) -> usize {
        util::term_width(self.chr)
    }

    pub fn overwrite(&mut self, s: term::Style, c: char) {
        use term::Style::{File, Directory};
        self.chr = c;
        self.brush.fg = match s {
            File => term::Color::new(200, 100, 100),
            Directory => term::Color::new(100, 100, 200),
        }
    }
}
