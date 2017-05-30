mod rect;
mod formatted;
mod color;
mod char;
mod line;
mod key;

pub use self::rect::Rect;
pub use self::char::Char;
pub use self::color::{Color, Brush};
pub use self::line::Line;
pub use self::formatted::{Style, Formatted};
pub use self::key::Key;

pub type Cursor = (usize, usize);

#[derive(Debug)]
pub struct Refresh {
    pub x: usize,
    pub y: usize,
    pub rect: Rect,
}
