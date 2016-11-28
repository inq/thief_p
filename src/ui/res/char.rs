use ui::{Style, Brush, Color};
use util;

#[derive(Clone, Debug)]
pub struct Char {
    pub chr: char,
    pub brush: Brush,
}

impl Char {
    pub fn new(chr: char, brush: Brush) -> Char {
        Char {
            chr: chr,
            brush: brush,
        }
    }

    pub fn width(&self) -> usize {
        util::term_width(self.chr)
    }

    pub fn overwrite(&mut self, s: Style, c: char) {
        self.chr = c;
        self.brush.fg = match s {
            Style::File => Color::new(200, 100, 100),
            Style::Directory => Color::new(100, 100, 200),
        }
    }
}
