use std::error;
use ui::buffer::Buffer;
use ui::color::{Brush, Color};
use ui::window::Window;

pub struct Screen {
    windows: Vec<(Window, usize, usize)>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let mut res = Screen {
            windows: vec![
                (Window::new(0, 0), 0, 0),
                (Window::new(0, 0), 0, 0)
            ],
            width: 0,
            height: 0,
        };
        res.resize(width, height);
        res
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let borders = self.windows.len() + 1;
        let windows = self.windows.len();
        let mut offset = 1;
        for (i, &mut(ref mut win, ref mut x, ref mut y)) in self.windows.iter_mut().enumerate() {
            let w = (self.width - borders + i + 1) / windows;
            win.resize(w, self.height - 2);
            *x = offset;
            *y = 1;
            offset += w + 1;
        }
    }

    pub fn refresh(&self, mut buf: &mut String) -> Result<(), Box<error::Error>> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        for &(ref win, x, y) in self.windows.iter() {
            buffer.draw(&win.refresh(), x, y);
        }

        buffer.print(&mut buf, &b.invert());
        Ok(())
    }
}
