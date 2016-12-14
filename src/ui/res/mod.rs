mod rect;
mod formatted;
mod color;
mod char;
mod line;

pub use self::rect::Rect;
pub use self::char::Char;
pub use self::color::{Color, Brush};
pub use self::line::Line;
pub use self::formatted::{Style, Formatted};
use common::Pair;

pub type Cursor = Pair;

#[derive(Default)]
pub struct Response {
    pub refresh: Option<Refresh>,
    pub sequence: Vec<Sequence>,
}

pub struct Refresh {
    pub x: usize,
    pub y: usize,
    pub rect: Rect,
}

#[derive(Debug)]
pub enum Sequence {
    Move(Cursor),
    Line(Line),
    Char(Char),
    Show(bool),
    Command(String),
    Quit,
}

impl Response {
    /// Shorthand for quit event.
    pub fn quit() -> Response {
        Response { sequence: vec![Sequence::Quit], ..Default::default() }
    }

    /// Shorthand for refresh.
    pub fn refresh(x: usize, y: usize, rect: Rect) -> Response {
        Response {
            refresh: Some(Refresh {
                x: x,
                y: y,
                rect: rect,
            }),
            ..Default::default()
        }
    }

    pub fn translate(mut self, tx: usize, ty: usize) -> Response {
        if let Some(Refresh { ref mut x, ref mut y, .. }) = self.refresh {
            *x += tx;
            *y += ty;
        }
        for r in self.sequence.iter_mut() {
            match *r {
                Sequence::Move(Cursor { ref mut x, ref mut y }) => {
                    *x += tx;
                    *y += ty;
                }
                _ => (),
            }
        }
        self
    }
}
