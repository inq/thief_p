use ui::color::Brush;

#[derive(Clone, Debug)]
pub struct Char {
    pub chr: char,
    pub brush: Brush,
}

extern "C" {
    fn wcwidth(chr: u32) -> u32;
}

impl Char {
    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        unsafe { wcwidth(self.chr as u32) as usize }
    }
}