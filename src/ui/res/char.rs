use ui::res::color::Brush;
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
}
