use msg::event;
use buf::Buffer;
use util::ResultBox;
use ui::comp::ViewT;
use ui::res::{self, Cursor, Line, Rect, Refresh};

#[allow(dead_code)]
#[derive(Default)]
pub struct LineEditor {
    view: ViewT,
    linenum_width: usize,
    x_offset: usize, // TODO: Implement this
}

pub enum Response {
    Ui(res::Response),
    LineBreak(Cursor),
    PullUp(usize),
    Move(Cursor, Cursor),
    Unhandled,
}

impl LineEditor {
    pub fn new() -> LineEditor {
        Default::default()
    }

    /// TODO: Cache the line
    pub fn set_linenum_width(&mut self, linenum_width: usize) {
        self.linenum_width = linenum_width;
    }

    /// Render to the Line object.
    /// TODO: Process long lines
    pub fn render(&self, buf: &mut Buffer, linenum: usize) -> ResultBox<Line> {
        let mut cache = Line::new_splitted(self.view.width,
                                           self.view.theme.linenum_cur(),
                                           self.view.theme.editor_cur(),
                                           self.linenum_width);
        cache.draw_str(&format!("{:width$}", linenum, width = self.linenum_width),
                       0,
                       0);
        if let Some(s) = buf.get(linenum) {
            cache.draw_str(s, self.linenum_width, 0);
        }
        Ok(cache)
    }

    /// Calculate the screen's coordinate of the cursor.
    #[inline]
    fn translate_cursor(&self, cursor: usize) -> usize {
        cursor + self.linenum_width
    }

    #[inline]
    fn spaces_after_cursor(&self, cursor: usize) -> usize {
        self.view.width - self.linenum_width - cursor
    }

    /// Delete the current character.
    fn on_delete(&mut self, buf: &mut Buffer) -> ResultBox<Response> {
        use buf::BackspaceRes::{Normal, PrevLine};
        let cursor = buf.get_x();
        match buf.backspace(self.spaces_after_cursor(cursor)) {
            Normal(mut after_cursor) => {
                after_cursor.push(' ');
                let line = Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
                self.response_cursor_with_line(buf.get_x(), line, true)
            }
            PrevLine(_cursor) => {
                // TODO: Implement this.
                Ok(Response::Unhandled)
            }
            _ => Ok(Response::Unhandled),
        }
    }

    /// Delete every characters after cursor.
    fn on_kill_line(&mut self, buf: &mut Buffer) -> ResultBox<Response> {
        use buf::KillLineRes::{Normal, Empty};
        let cursor = buf.get_x();
        match buf.kill_line() {
            Normal => {
                let line = Line::new_from_str(&vec![' '; self.spaces_after_cursor(cursor)]
                                                  .into_iter()
                                                  .collect::<String>(),
                                              self.view.theme.editor);
                self.response_cursor_with_line(buf.get_x(), line, false)
            }
            Empty(cursor) => Ok(Response::PullUp(cursor.y)),
            _ => Ok(Response::Unhandled),
        }
    }

    /// Update the y and width.
    pub fn resize(&mut self, y: usize, width: usize) -> ResultBox<()> {
        self.view.y = y;
        self.view.width = width;
        Ok(())
    }

    /// Handle move events.
    fn on_move(&mut self, buf: &mut Buffer, dx: i8, dy: i8) -> ResultBox<Response> {
        let cursor_prev = buf.get_cursor();
        let cursor = buf.move_cursor(dx, dy);
        if cursor_prev.y == cursor.y {
            // Move only in here.
            self.response_cursor(cursor.x)
        } else {
            // Pass the process to the parrent.
            Ok(Response::Move(cursor_prev, cursor))
        }
    }

    /// Response wrapper for UI
    #[inline]
    fn response_ui(&self, response: res::Response) -> ResultBox<Response> {
        Ok(Response::Ui(response))
    }

    /// Response with cursor.
    fn response_cursor(&self, cursor: usize) -> ResultBox<Response> {
        self.response_ui(res::Response::Term {
            cursor: Some(Cursor {
                x: self.translate_cursor(cursor),
                y: 0,
            }),
            refresh: None,
        })
    }

    /// Response with current cursor and following line.
    fn response_cursor_with_line(&self,
                                 cursor: usize,
                                 line: Line,
                                 on_delete: bool)
                                 -> ResultBox<Response> {
        let x = self.translate_cursor(cursor);
        self.response_ui(res::Response::Term {
            refresh: Some(Refresh {
                x: if on_delete { x } else { x - 1 },
                y: 0,
                rect: Rect::new_from_line(line),
            }),
            cursor: Some(Cursor { x: x, y: 0 }),
        })
    }

    /// Move cursor left and right, or Type a character.
    pub fn on_key(&mut self, buf: &mut Buffer, k: event::Key) -> ResultBox<Response> {
        match k {
            event::Key::Ctrl('a') |
            event::Key::Home => {
                let cursor = buf.move_begin_of_line().x;
                self.response_cursor(cursor)
            }
            event::Key::Ctrl('e') |
            event::Key::End => {
                let cursor = buf.move_end_of_line().x;
                self.response_cursor(cursor)
            }
            event::Key::CR => {
                let cursor = buf.break_line();
                Ok(Response::LineBreak(cursor))
            }
            event::Key::Del => self.on_delete(buf),
            event::Key::Ctrl('k') => self.on_kill_line(buf),
            event::Key::Ctrl('n') |
            event::Key::Down => self.on_move(buf, 0, 1),
            event::Key::Ctrl('p') |
            event::Key::Up => self.on_move(buf, 0, -1),
            event::Key::Ctrl('f') |
            event::Key::Right => self.on_move(buf, 1, 0),
            event::Key::Ctrl('b') |
            event::Key::Left => self.on_move(buf, -1, 0),
            event::Key::Char(c) => {
                let mut after_cursor = String::with_capacity(self.view.width);
                after_cursor.push(c);
                let cursor = buf.get_x();
                after_cursor.push_str(&buf.insert(c, self.spaces_after_cursor(cursor)));
                let cursor = buf.get_x();
                let line = Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
                self.response_cursor_with_line(cursor, line, false)
            }
            _ => Ok(Response::Unhandled),
        }
    }
}
