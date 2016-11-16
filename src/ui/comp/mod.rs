mod screen;
mod window;
mod command_bar;
mod hsplit;

pub use self::command_bar::CommandBar;
pub use self::screen::Screen;
pub use self::window::Window;
pub use self::hsplit::HSplit;

use ui::res::{Buffer, Cursor, Response, Refresh, Sequence};
use io::Event;


use std::io::{stderr, Write};

#[derive(Default)]
pub struct View {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub trait Component {
    fn get_view(&self) -> &View;
    fn resize(&mut self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize);
    fn refresh(&self) -> Response;
    fn handle(&mut self, _: Event) -> Response {
        Default::default()
    }
    /// Apply offset of the child to the responses.
    fn transform(&self, resp: Response) -> Response {
        resp.translate(self.get_view().x, self.get_view().y)
    }
}

pub trait Parent {
    type Child: Component;
    fn children_mut(&mut self) -> Vec<&mut Self::Child>;
    fn children(&self) -> Vec<&Self::Child>;

    /// Draw the children and transform each sequenced results.
    fn refresh_children(&self, buffer: Buffer) -> Response {
        let mut refresh = Refresh { x: 0, y: 0, buf: buffer };
        let mut sequence = vec![];
        for &ref child in self.children() {
            let resp = child.refresh();
            if let Some(Refresh { x, y, buf }) = resp.refresh {
                refresh.buf.draw(&buf, child.get_view().x + x, child.get_view().y + y)
            }
            for r in resp.sequence {
                match r {
                    Sequence::Move(cur) => {
                        sequence.push(Sequence::Move(Cursor {
                            x: cur.x + child.get_view().x,
                            y: cur.y + child.get_view().y,
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
