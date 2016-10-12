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
    Refresh(usize, usize, Buffer),
    Move(Cursor),
    Put(String),
    Quit,
}

pub trait Trans {
    fn translate(mut self, x: usize, y: usize) -> Self;
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
