use std::path::Path;
use buf;
use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::{Component, Child};
use util::ResultBox;

pub struct Editor {
    buffer: buf::Buffer,
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.width = width;
        self.height = height;
        (width, height)
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
    #[allow(dead_code)]
    pub fn new() -> Child {
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(Editor {
                buffer: buf::Buffer::new(),
                width: usize::max_value(),
                height: usize::max_value(),
            }),
        }
    }

    /// Initializer with file.
    pub fn new_with_file<S: AsRef<Path> + ?Sized>(s: &S) -> ResultBox<Child> {
        let mut editor = Editor {
            buffer: buf::Buffer::new(),
            width: usize::max_value(),
            height: usize::max_value(),
        };
        try!(editor.load_file(s));
        Ok(Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(editor),
        })
    }

    fn load_file<S: AsRef<Path> + ?Sized>(&mut self, s: &S) -> ResultBox<()> {
        self.buffer.load_file(s)
    }
}
