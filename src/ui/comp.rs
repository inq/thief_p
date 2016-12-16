use hq::Hq;
use ui::res::{Cursor, Rect, Response, Refresh, Sequence};
use io::Event;
use util::ResultBox;

#[derive(Default)]
pub struct View {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl View {
    fn update(&mut self, x: usize, y: usize, width: usize, height: usize) {
        *self = View {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

pub trait Component {
    fn get_view(&self) -> &View;
    fn get_view_mut(&mut self) -> &mut View;
    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()>;
    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response>;
    /// Resize the component; Call the on_resize function.
    fn resize(&mut self,
              hq: &mut Hq,
              x: usize,
              y: usize,
              width: usize,
              height: usize)
              -> ResultBox<()> {
        self.get_view_mut().update(x, y, width, height);
        self.on_resize(hq)
    }
    /// Handle the given event.
    fn handle(&mut self, _: Event, _: &mut Hq) -> ResultBox<Response> {
        Ok(Default::default())
    }
    /// Propage event to children. This calls handle, and then translate.
    fn propagate(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        Ok(self.handle(e, hq)?.translate(self.get_view().x, self.get_view().y))
    }
}

pub trait Parent {
    type Child: Component;
    fn children_mut(&mut self) -> Vec<&mut Self::Child>;
    fn children(&self) -> Vec<&Self::Child>;

    /// Draw the children and transform each sequenced results.
    fn refresh_children(&mut self, rect: Rect, hq: &mut Hq) -> ResultBox<Response> {
        let mut refresh = Refresh {
            x: 0,
            y: 0,
            rect: rect,
        };
        let mut sequence = vec![];
        for ref mut child in self.children_mut() {
            let resp = child.refresh(hq)?;
            if let Some(Refresh { x, y, rect }) = resp.refresh {
                refresh.rect.draw(&rect, child.get_view().x + x, child.get_view().y + y)
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
        Ok(Response {
            refresh: Some(refresh),
            sequence: sequence,
        })
    }
}
