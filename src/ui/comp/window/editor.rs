use std::path::Path;
use buf;
use io::Event;
use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::{Component, Child};
use util::ResultBox;
use super::LineNumber;

pub struct Editor {
    line_number: LineNumber,
    buffer: buf::Buffer,
    cursor: Cursor,
    x_off: usize,
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.x_off = self.line_number.resize(usize::max_value(), height).0 + 1;
        self.width = width;
        self.height = height;
        (width, height)
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        // Draw line_number
        for resp in self.line_number.refresh() {
            match resp {
                Response::Refresh(x, y, buf) => buffer.draw(&buf, 0 + x, 0 + y),
                _ => (),
            }
        }
        // Draw the others
        buffer.draw_buffer(&self.buffer, self.x_off, 0);
        vec![
            Response::Refresh(
                0, 0,
                buffer,
            ),
            Response::Move(Cursor { x: 0, y: 0 }),
        ]
    }

    /// Move cursor left and right.
    fn handle(&mut self, e: Event) -> Vec<Response> {
        match e {
            Event::Move { x: x, y: 0 } => {
                self.cursor.x = if x > 0 {
                    self.cursor.x + 1
                } else {
                    if self.cursor.x > 0 {
                        self.cursor.x - 1
                    } else {
                        self.cursor.x
                    }
                };
                let mut cur = self.cursor.clone();
                cur.x += self.x_off;
                vec![Response::Move(cur)]
            }
            _ => vec![]
        }
    }
}

impl Editor {
    #[allow(dead_code)]
    pub fn new() -> Child {
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(Editor {
                line_number: LineNumber::new(),
                cursor: Cursor { x: usize::max_value(), y: usize::max_value() },
                buffer: buf::Buffer::new(),
                x_off: usize::max_value(),
                width: usize::max_value(),
                height: usize::max_value(),
            }),
        }
    }

    /// Initializer with file.
    pub fn new_with_file<S: AsRef<Path> + ?Sized>(s: &S) -> ResultBox<Child> {
        let mut editor = Editor {
            line_number: LineNumber::new(),
            cursor: Cursor { x: usize::max_value(), y: usize::max_value() },
            buffer: buf::Buffer::new(),
            x_off: usize::max_value(),
            width: usize::max_value(),
            height: usize::max_value(),
        };
        try!(editor.load_file(s));
        editor.cursor = Cursor { x: 0, y: 0 };
        editor.line_number.set_max(100);
        Ok(Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(editor),
        })
    }

    fn load_file<S: AsRef<Path> + ?Sized>(&mut self, s: &S) -> ResultBox<()> {
        self.buffer = try!(buf::Buffer::from_file(s));
        Ok(())
    }
}
