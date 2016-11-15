mod buffer;
mod color;
mod char;
mod line;

pub use self::buffer::Buffer;
pub use self::char::Char;
pub use self::color::{Color, Brush};
pub use self::line::Line;

#[derive(Debug, Default, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
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
    Line(Line),
    Show(bool),
    Quit,
}

impl Response {
    /// Shorthand for quit event.
    pub fn quit() -> Response {
        Response {
            sequence: vec![Sequence::Quit],
            ..Default::default()
        }
    }

    /// Shorthand for refresh.
    pub fn refresh(x: usize, y: usize, buf: Buffer) -> Response {
        Response {
            refresh: Some(Refresh { x: x, y: y, buf: buf }),
            ..Default::default()
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
