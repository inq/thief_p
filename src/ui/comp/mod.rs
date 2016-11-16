mod screen;
mod window;
mod command_bar;
mod hsplit;

pub use self::command_bar::CommandBar;
pub use self::screen::Screen;
pub use self::window::EditWindow;
pub use self::hsplit::HSplit;

use ui::res::{Buffer, Cursor, Response, Refresh, Sequence};
use io::Event;

pub trait Component {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize);
    fn refresh(&self) -> Response;
    fn handle(&mut self, _: Event) -> Response {
        Default::default()
    }
}

pub trait Parent {
    fn children_mut(&mut self) -> Vec<&mut Child>;
    fn children(&self) -> Vec<&Child>;

    /// Apply offset of the child to the responses.
    fn transform(&self, child: &Child, mut resp: Response) -> Response {
        if let Some(Refresh { ref mut x, ref mut y, .. }) = resp.refresh {
            *x += child.x;
            *y += child.y;
        }
        for r in resp.sequence.iter_mut() {
            match *r {
                Sequence::Move(Cursor { ref mut x, ref mut y }) => {
                    *x += child.x;
                    *y += child.y;
                }
                _ => (),
            }
        }
        resp
    }

    /// Draw the children and transform each sequenced results.
    fn refresh_children(&self, buffer: Buffer) -> Response {
        let mut refresh = Refresh { x: 0, y: 0, buf: buffer };
        let mut sequence = vec![];
        for &ref child in self.children() {
            let resp = child.comp.refresh();
            if let Some(Refresh { x, y, buf }) = resp.refresh {
                refresh.buf.draw(&buf, child.x + x, child.y + y)
            }
            for r in resp.sequence {
                match r {
                    Sequence::Move(cur) => {
                        sequence.push(Sequence::Move(Cursor {
                            x: cur.x + child.x,
                            y: cur.y + child.y,
                        }));
                    }
                    x => {
                        sequence.push(x);
                    }
                }
            }
        }
        Response {
            refresh: Some(refresh),
            sequence: sequence,
        }
    }
}

#[derive(Default)]
pub struct View {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Child {
    pub comp: Box<Component>,
    pub x: usize,
    pub y: usize,
}

impl Child {
    pub fn new(b: Box<Component>) -> Child {
        Child {
            comp: b,
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn refresh(&self) -> Response {
        let mut resp = self.comp.refresh();
        resp.translate(self.x, self.y);
        resp
    }
}
