use std::error;
use ui::prim::{term, Buffer, Brush, Color};
use ui::window::Window;
use ui::comp::{Child, Component, Cursor, Response};

pub struct Screen {
    windows: Vec<Child<Window>>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let mut res = Screen {
            windows: vec![
                Child { comp: Window::new(0, 0), x: 0, y: 0 },
                Child { comp: Window::new(0, 0), x: 0, y: 0 },
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
        for (i, &mut ref mut child) in self.windows.iter_mut().enumerate() {
            let w = (self.width - borders + i + 1) / windows;
            child.comp.resize(w, self.height - 2);
            child.x = offset;
            child.y = 1;
            offset += w + 1;
        }
    }

    pub fn refresh(&self, mut buf: &mut String) -> Result<(), Box<error::Error>> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut cursor = None;
        for &ref child in self.windows.iter() {
            let r = child.comp.refresh();
            if let Some(buf) = r.draw {
                buffer.draw(&buf, child.x, child.y);
            }
            if let Some(cur) = r.cursor {
                cursor = Some(Cursor { x: cur.x + child.x, y: cur.y + child.y } );
            }
        }
        buffer.print(&mut buf, &b.invert());
        if let Some(cur) = cursor {
            term::movexy(&mut buf, cur.x, cur.y);
        }
        Ok(())
    }
}
