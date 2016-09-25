use ui::prim::{Buffer, Brush, Color};

pub struct Window {
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        Window { width: width, height: height }
    }
}

impl Window {
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn refresh(&self) -> Buffer {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 200, 200));
        Buffer::bordered(&b, &b.invert(), self.width, self.height)
    }
}
