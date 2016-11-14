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

pub struct Response {
    pub refresh: Option<Refresh>,
    pub sequence: Vec<Sequence>,
}

pub struct Refresh {
    pub x: usize,
    pub y: usize,
    pub buf: Buffer,
}

#[derive(Debug)]
pub enum Sequence {
    Move(Cursor),
    Put(String),
    Line(Line),
    Show(bool),
    Quit,
}

impl Response {
    /// Empty response.
    pub fn empty() -> Response {
        Response { refresh: None, sequence: vec![] }
    }

    /// Shorthand for quit event.
    pub fn quit() -> Response {
        Response {
            refresh: None,
            sequence: vec![Sequence::Quit],
        }
    }

    /// Shorthand for refresh.
    pub fn refresh(x: usize, y: usize, buf: Buffer) -> Response {
        Response {
            refresh: Some(Refresh { x: x, y: y, buf: buf }),
            sequence: vec![],
        }
    }

    pub fn translate(&mut self, tx: usize, ty: usize) {
        if let Some(Refresh { ref mut x, ref mut y, .. }) = self.refresh {
            *x += tx;
            *y += ty;
        }
        for seq in self.sequence.iter_mut() {
            match *seq {
                Sequence::Move(Cursor{ref mut x, ref mut y}) => {
                    *x += tx;
                    *y += ty;
                }
                _ => (),
            }
        }
    }
}
