mod buffer;
mod color;
mod char;
mod line;

pub use ui::res::buffer::Buffer;
pub use ui::res::char::Char;
pub use ui::res::color::{Color, Brush};
pub use ui::res::line::Line;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub enum Response {
    Refresh(Buffer),
    Move(Cursor),
    Put(String),
    Quit,
}
