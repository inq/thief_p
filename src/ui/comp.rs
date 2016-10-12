use ui::res::{Buffer, Cursor, Response};
use ui::res::Trans;

pub trait Component {
    fn resize(&mut self, width: usize, height: usize);
    fn refresh(&self) -> Vec<Response>;
}

pub trait Parent {
    fn children_mut(&mut self) -> Vec<&mut Child>;
    fn children(&self) -> Vec<&Child>;

    fn refresh_children(&self, buffer: &mut Buffer) -> Vec<Response> {
        let mut cursor = None;
        for &ref child in self.children() {
            for resp in child.comp.refresh() {
                match resp {
                    Response::Refresh(x, y, buf) => {
                        buffer.draw(&buf, child.x + x, child.y + y)
                    }
                    Response::Move(cur) => {
                        cursor = Some(Cursor {
                            x: cur.x + child.x,
                            y: cur.y + child.y,
                        })
                    }
                    _ => (),
                }
            }
        }
        let mut res = vec![];
        if let Some(cur) = cursor {
            res.push(Response::Move(cur));
        }
        res
    }
}

pub struct Child {
    pub comp: Box<Component>,
    pub x: usize,
    pub y: usize,
}

impl Child {
    pub fn refresh(&self) -> Vec<Response> {
        let mut res = vec![];
        res.append(&mut self.comp.refresh());
        res.translate(self.x, self.y)
    }
}
