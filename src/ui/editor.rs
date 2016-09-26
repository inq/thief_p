use ui::prim::{Buffer, Brush, Color};
use ui::comp::{Component, Child};

pub struct Editor {
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
}

impl Editor {
    pub fn new(width: usize, height: usize) -> Editor {
        Editor {
            width: width,
            height: height,
        }
    }

    pub fn refresh(&self) -> Buffer {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220));
        Buffer::bordered(&b, &b.invert(), self.width, self.height)
    }
}