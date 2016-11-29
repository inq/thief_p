use hq::Hq;
use io::Event;
use util::ResultBox;
use buf::{BackspaceRes, KillLineRes};
use ui::res::{Buffer, Brush, Color, Cursor, Line, Response, Refresh, Sequence};
use ui::comp::{Component, View};
use super::LineNumber;

#[derive(Default)]
pub struct Editor {
    view: View,
    line_number: LineNumber,
    buffer_name: String,
    cursor: Cursor,
    x_off: usize,
    brush: Brush,
}

impl Component for Editor {
    has_view!();

    fn on_resize(&mut self) {
        self.line_number.set_max(100);
        self.line_number.resize(0, 0, Default::default(), self.view.height);
        self.x_off = self.line_number.get_view().width + 1;
    }

    fn refresh(&self, hq: &mut Hq) -> ResultBox<Response> {
        // TODO: implement
        let mut buffer = Buffer::blank(&self.brush, self.view.width, self.view.height);
        // Draw line_number
        if let Some(Refresh { x, y, buf }) = self.line_number.refresh(hq)?.refresh {
            buffer.draw(&buf, 0 + x, 0 + y);
        }
        // Draw the others
        buffer.draw_buffer(hq.buf(&self.buffer_name)?,
                           self.x_off,
                           0,
                           self.line_number.current);
        Ok(Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                buf: buffer,
            }),
            sequence: vec![Sequence::Show(true), self.move_cursor()],
        })
    }

    /// Move cursor left and right, or Type a character.
    fn handle(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        match e {
            Event::OpenBuffer { s } => {
                self.buffer_name = s;
                Ok(Default::default())
            }
            Event::Single { n: 1 } |
            Event::Ctrl { c: 'a' } => {
                // HOME
                self.cursor = hq.buf(&self.buffer_name)?.move_begin_of_line();
                Ok(Response { sequence: vec![self.move_cursor()], ..Default::default() })
            }
            Event::Single { n: 4 } |
            Event::Ctrl { c: 'e' } => {
                // END
                self.cursor = hq.buf(&self.buffer_name)?.move_end_of_line();
                Ok(Response { sequence: vec![self.move_cursor()], ..Default::default() })
            }
            Event::Ctrl { c: 'n' } => self.handle(Event::Move { x: 0, y: 1 }, hq),
            Event::Ctrl { c: 'p' } => self.handle(Event::Move { x: 0, y: -1 }, hq),
            Event::Ctrl { c: 'f' } => self.handle(Event::Move { x: 1, y: 0 }, hq),
            Event::Ctrl { c: 'b' } => self.handle(Event::Move { x: -1, y: 0 }, hq),
            Event::Move { x, y } => {
                self.cursor = hq.buf(&self.buffer_name)?.move_cursor(x, y);
                if self.cursor.y < self.line_number.current {
                    // Scroll upward
                    self.line_number.current = self.cursor.y;
                    self.refresh(hq)
                } else if self.cursor.y - self.line_number.current >= self.view.height {
                    // Scroll downward
                    self.line_number.current = self.cursor.y - self.view.height + 1;
                    self.refresh(hq)
                } else {
                    // Do not scroll
                    Ok(Response { sequence: vec![self.move_cursor()], ..Default::default() })
                }
            }
            Event::Ctrl { c: 'm' } => {
                // CR
                self.cursor = hq.buf(&self.buffer_name)?.break_line();
                self.refresh(hq)
            }
            Event::Ctrl { c: 'k' } => {
                // Kill-line
                match hq.buf(&self.buffer_name)?.kill_line() {
                    KillLineRes::Normal => {
                        let blanks = vec![' '; self.spaces_after_cursor()]
                            .into_iter()
                            .collect::<String>();
                        Ok(Response {
                            sequence: vec![Sequence::Show(false),
                                           Sequence::Line(Line::new_from_str(&blanks,
                                                                             &self.brush)),
                                           Sequence::Move(self.cursor_translated()),
                                           Sequence::Show(true)],
                            ..Default::default()
                        })
                    }
                    KillLineRes::Empty(cursor) => {
                        self.cursor = cursor;
                        self.refresh(hq)
                    }
                    _ => Ok(Default::default()),
                }
            }
            Event::Char { c: '\x7f' } => {
                // Backspace
                match hq.buf(&self.buffer_name)?.backspace(self.spaces_after_cursor()) {
                    BackspaceRes::Normal(mut after_cursor) => {
                        after_cursor.push(' ');
                        self.cursor.x -= 1;
                        Ok(Response {
                            sequence: vec![Sequence::Show(false),
                                           Sequence::Move(self.cursor_translated()),
                                           Sequence::Line(Line::new_from_str(&after_cursor,
                                                                             &self.brush)),
                                           Sequence::Move(self.cursor_translated()),
                                           Sequence::Show(true)],
                            ..Default::default()
                        })
                    }
                    BackspaceRes::PrevLine(cursor) => {
                        self.cursor = cursor;
                        self.refresh(hq)
                    }
                    _ => Ok(Default::default()),
                }
            }
            Event::Char { c } => {
                let mut after_cursor = String::with_capacity(self.view.width);
                self.cursor.x += 1;
                after_cursor.push(c);
                after_cursor.push_str(&hq.buf(&self.buffer_name)?
                    .insert(c, self.spaces_after_cursor()));
                Ok(Response {
                    sequence: vec![Sequence::Show(false),
                                   Sequence::Line(Line::new_from_str(&after_cursor, &self.brush)),
                                   Sequence::Move(self.cursor_translated()),
                                   Sequence::Show(true)],
                    ..Default::default()
                })
            }
            _ => Ok(Default::default()),
        }
    }
}

impl Editor {
    #[inline]
    fn spaces_after_cursor(&self) -> usize {
        self.view.width - self.x_off - self.cursor.x
    }

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
            buffer_name: String::from("<empty>"),
            brush: Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220)),
            ..Default::default()
        }
    }
}
