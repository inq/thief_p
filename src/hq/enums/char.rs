use term;
use util;

#[derive(Clone, Debug)]
pub struct Char {
    pub chr: char,
    pub brush: term::Brush,
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
