use std::path::Path;
use buf;
use io::Event;
use ui::res::{Buffer, Brush, Color, Cursor, Line, Response, Refresh, Sequence};
use ui::comp::{Component, View};
use util::ResultBox;
use super::LineNumber;

#[derive(Default)]
pub struct Editor {
    view: View,
    line_number: LineNumber,
    buffer: buf::Buffer,
    cursor: Cursor,
    x_off: usize,
    brush: Brush,
}

impl Component for Editor {
    has_view!();

    fn on_resize(&mut self) {
        self.line_number.resize(0, 0, Default::default(), self.view.height);
        self.x_off = self.line_number.get_view().width + 1;
    }

    fn refresh(&self) -> Response {
        let mut buffer = Buffer::blank(&self.brush, self.view.width, self.view.height);
        // Draw line_number
        if let Some(Refresh { x, y, buf }) = self.line_number.refresh().refresh {
            buffer.draw(&buf, 0 + x, 0 + y);
        }
        // Draw the others
        buffer.draw_buffer(&self.buffer, self.x_off, 0, self.line_number.current);
        Response {
            refresh: Some(Refresh { x: 0, y: 0, buf: buffer }),
            sequence: vec![
                Sequence::Show(true),
                self.move_cursor(),
            ]
        }
    }

    /// Move cursor left and right, or Type a character.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Move { x, y } => {
                self.buffer.move_cursor(x, y);
                self.cursor.x = self.buffer.get_x();
                self.cursor.y = self.buffer.get_y();
                if self.cursor.y < self.line_number.current {
                    // Scroll upward
                    self.line_number.current = self.cursor.y;
                    self.refresh()
                } else if self.cursor.y - self.line_number.current >= self.view.height {
                    // Scroll downward
                    self.line_number.current = self.cursor.y - self.view.height + 1;
                    self.refresh()
                } else {
                    // Do not scroll
                    Response {
                        sequence: vec![self.move_cursor()],
                        ..Default::default()
                    }
                }
            }
            Event::Ctrl { c: 'm' } => { // CR
                self.buffer.break_line();
                self.cursor.x = self.buffer.get_x();
                self.cursor.y = self.buffer.get_y();
                self.refresh()
            }
            Event::Char { c } => {
                self.buffer.insert(c);
                let req = self.view.width - self.x_off - self.cursor.x;
                let mut after_cursor = String::with_capacity(self.view.width);
                self.cursor.x += 1;
                after_cursor.push(c);
                after_cursor.push_str(&self.buffer.after_cursor(req));
                Response {
                    sequence: vec![
                        Sequence::Show(false),
                        Sequence::Line(Line::new_from_str(&after_cursor, &self.brush)),
                        Sequence::Move(self.cursor_translated()),
                        Sequence::Show(true),
                    ],
                    ..Default::default()
                }
            }
            _ => Default::default()
        }
    }
}

impl Editor {
    fn cursor_translated(&self) -> Cursor {
        let mut cur = self.cursor.clone();
        cur.x += self.x_off;
        cur
    }

    fn move_cursor(&self) -> Sequence {
        Sequence::Move(Cursor {
            x: self.cursor.x + self.x_off,
            y: self.cursor.y - self.line_number.current,
        })
    }

    /// Basic initializer.
    pub fn new() -> Editor {
        Editor {
            brush: Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220)),
            ..Default::default()
        }
    }

    /// Initializer with file.
    pub fn new_with_file<S: AsRef<Path> + ?Sized>(s: &S) -> ResultBox<Editor> {
        let mut editor = Editor::new();
        editor.load_file(s)?;
        editor.cursor = Cursor { x: 0, y: 0 };
        editor.line_number.set_max(100);
        editor.line_number.current = 0;
        editor.buffer.set_cursor(0, 0);
        Ok(editor)
    }

    fn load_file<S: AsRef<Path> + ?Sized>(&mut self, s: &S) -> ResultBox<()> {
        self.buffer = buf::Buffer::from_file(s)?;
        Ok(())
    }
}
