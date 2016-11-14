mod buffer;
mod color;
mod char;
mod line;
mod cursor;

pub use self::buffer::Buffer;
pub use self::char::Char;
pub use self::color::{Color, Brush};
pub use self::line::Line;
pub use self::cursor::Cursor;

#[derive(Debug)]
pub enum Response {
    Refresh(usize, usize, Buffer),
    Move(Cursor),
    Put(String),
    Line(Line),
    Show(bool),
    Quit,
}

pub trait Trans {
    fn translate(self, x: usize, y: usize) -> Self;
}

impl Trans for Vec<Response> {
    fn translate(self, tx: usize, ty: usize) -> Self {
        self.into_iter().map(|i| match i {
            Response::Refresh(x, y, b) => Response::Refresh(x + tx, y + ty, b),
            Response::Move(Cursor{x, y}) => Response::Move(Cursor{x: x + tx, y: y + ty}),
            x => x
        }).collect()
    }
}
