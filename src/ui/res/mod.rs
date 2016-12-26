mod rect;
mod formatted;
mod color;
mod char;
mod line;
mod text_rect;

pub use self::rect::Rect;
pub use self::char::Char;
pub use self::color::{Color, Brush};
pub use self::line::Line;
pub use self::formatted::{Style, Formatted};
pub use self::text_rect::TextRect;
use msg::Pair;

pub type Cursor = Pair;

pub enum Response {
    Unhandled,
    Command(String),
    Quit,
    Term {
        refresh: Option<Refresh>,
        cursor: Option<Cursor>,
    },
}

impl Default for Response {
    fn default() -> Response {
        Response::Term {
            refresh: None,
            cursor: None,
        }
    }
}

pub struct Refresh {
    pub x: usize,
    pub y: usize,
    pub rect: Rect,
}

#[derive(Debug)]
pub enum Sequence {
    Unhandled,
    Command(String),
    Quit,
}

impl Response {
    #[inline]
    pub fn is_handled(&self) -> bool {
        if let Response::Unhandled = *self {
            false
        } else {
            true
        }
    }

    pub fn translate(mut self, tx: usize, ty: usize) -> Response {
        if let Response::Term { ref mut refresh, ref mut cursor } = self {
            if let Some(Refresh { ref mut x, ref mut y, .. }) = *refresh {
                *x += tx;
                *y += ty;
            }
            if let Some(Cursor { ref mut x, ref mut y }) = *cursor {
                *x += tx;
                *y += ty;
            }
        }
        self
    }
}
