use std::error;
use ui::buffer::Buffer;
use ui::color::{Brush, Color};

pub struct Screen {
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen { width: width, height: height }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn refresh(&self, mut buf: &mut String) -> Result<(), Box<error::Error>> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));

        let buffer = Buffer::bordered(&b, &b.invert(), self.width, self.height);
        buffer.print(&mut buf, &b.invert());
        Ok(())
    }
}
