use ui::color::Brush;
use libc;

#[derive(Clone, Debug)]
pub struct Char {
    pub chr: char,
    pub brush: Brush,
}

extern {
    fn wcwidth(chr: u32) -> u32;
}

impl Char {
    pub fn width(&self) -> u32 {
        unsafe { wcwidth(self.chr as u32) }
    }
}
