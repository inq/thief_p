#[macro_use]
mod macros;

pub type ResultBox<T> = ::std::result::Result<T, Box<::std::error::Error>>;

extern "C" {
    fn wcwidth(chr: u32) -> u32;
}

pub fn term_width(c: char) -> usize {
    unsafe { wcwidth(c as u32) as usize }
}
