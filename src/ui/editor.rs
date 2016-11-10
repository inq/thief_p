use std::path::Path;
use buf;
use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::Component;
use util::ResultBox;

pub struct Editor {
    buffer: buf::Buffer,
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        buffer.draw_buffer(&self.buffer, 0, 0);
        vec![
            Response::Refresh(
                0, 0,
                buffer,
            ),
            Response::Move(Cursor { x: 0, y: 0 }),
        ]
    }
}

impl Editor {
    pub fn new(width: usize, height: usize) -> Editor {
        Editor {
            buffer: buf::Buffer::new(),
            width: width,
            height: height,
        }
    }

    pub fn load_file<S: AsRef<Path> + ?Sized>(&mut self, s: &S) -> ResultBox<()> {
        self.buffer.load_file(s)
    }
}
