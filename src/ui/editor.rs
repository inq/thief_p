use ui::prim::{Buffer, Brush, Color};
use ui::comp::{Component, Child, Response, Cursor};

pub struct Editor {
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220));
        Response {
            draw: Some(Buffer::bordered(&b, &b.invert(), self.width, self.height)),
            cursor: Some(Cursor { x: 0, y: 0 }),
        }
    }
}

impl Editor {
    pub fn new(width: usize, height: usize) -> Editor {
        Editor {
            width: width,
            height: height,
        }
    }

}
