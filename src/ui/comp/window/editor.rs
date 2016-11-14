use std::path::Path;
use buf;
use io::Event;
use ui::res::{Buffer, Brush, Color, Cursor, Line, Response, Refresh, Sequence};
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
    brush: Brush,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.x_off = self.line_number.resize(usize::max_value(), height).0 + 1;
        self.width = width;
        self.height = height;
        (width, height)
    }

    fn refresh(&self) -> Response {
        let mut buffer = Buffer::blank(&self.brush, self.width, self.height);
        // Draw line_number
        if let Some(Refresh { x, y, buf }) = self.line_number.refresh().refresh {
            buffer.draw(&buf, 0 + x, 0 + y);
        }
        // Draw the others
        buffer.draw_buffer(&self.buffer, self.x_off, 0);
        Response {
            refresh: Some(Refresh { x: 0, y: 0, buf: buffer }),
            sequence: vec![
                Sequence::Show(true),
                Sequence::Move(Cursor { x: self.x_off, y: 0 }),
            ]
        }
    }

    /// Move cursor left and right, or Type a character.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Move { x, y: 0 } => {
                self.cursor.x = if x > 0 {
                    self.cursor.x + 1
                } else {
                    if self.cursor.x > 0 {
                        self.cursor.x - 1
                    } else {
                        self.cursor.x
                    }
                };
                self.buffer.move_cursor(x, 0);
                let mut cur = self.cursor.clone();
                cur.x += self.x_off;
                Response {
                    refresh: None,
                    sequence: vec![Sequence::Move(cur)],
                }
            }
            Event::Char { c } => {
                self.buffer.insert(c);
                let req = self.width - self.x_off - self.cursor.x;
                Response {
                    refresh: None,
                    sequence: vec![
                        Sequence::Show(false),
                        Sequence::Line(Line::new_from_str(&self.buffer.after_cursor(req), &self.brush)),
                        Sequence::Move(self.cursor_translated()),
                        Sequence::Show(true),
                    ]
                }
            }
            _ => Response::empty()
        }
    }
}

impl Editor {
    fn cursor_translated(&self) -> Cursor {
        let mut cur = self.cursor.clone();
        cur.x += self.x_off;
        cur
    }

    #[allow(dead_code)]
    pub fn new() -> Child {
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(Editor {
                brush: Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220)),
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
            brush: Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220)),
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
        editor.buffer.set_cursor(0, 0);
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
