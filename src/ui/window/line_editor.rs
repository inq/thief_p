use msg::event;
use hq::Hq;
use util::ResultBox;
use buf::{self, BackspaceRes};
use ui::comp::{Component, ViewT};
use ui::res::{self, Cursor, Line, Rect, Refresh};

#[derive(Default, UiView)]
pub struct LineEditor {
    view: ViewT,
    buffer_name: String,
    cursor: usize,
    linenum_width: usize,
}

pub enum Response {
    Ui(res::Response),
    LineBreak,
    Unhandled,
}

impl LineEditor {
    pub fn set_buffer_name(&mut self, buffer_name: &str) {
        self.buffer_name = String::from(buffer_name);
    }

    pub fn new(buffer_name: &str) -> LineEditor {
        LineEditor { buffer_name: String::from(buffer_name), ..Default::default() }
    }

    /// TODO: Cache the line
    pub fn set_linenum_width(&mut self, linenum_width: usize) {
        self.linenum_width = linenum_width;
    }

    /// Render to the Line object.
    /// TODO: Process long lines
    pub fn render(&self, hq: &mut Hq, line: usize) -> ResultBox<Line> {
        let buf = hq.buf(&self.buffer_name)?;
        let mut cache = Line::new_splitted(self.view.width,
                                           self.view.theme.linenum_cur(),
                                           self.view.theme.editor_cur(),
                                           self.linenum_width);
        cache.render_buf(buf, line);
        Ok(cache)
    }

    /// Calculate the screen's coordinate of the cursor.
    #[inline]
    fn cursor_translated(&self) -> usize {
        self.cursor + self.linenum_width
    }

    #[inline]
    fn gen_cur_line(&self, hq: &mut Hq) -> Line {
        let buf = hq.buf(&self.buffer_name).unwrap();
        let mut cache = Line::new_splitted(self.view.width,
                                           self.view.theme.linenum_cur(),
                                           self.view.theme.editor_cur(),
                                           self.linenum_width);
        cache.render_buf(buf, 0);
        cache
    }

    #[inline]
    fn spaces_after_cursor(&self) -> usize {
        self.view.width - self.linenum_width - self.cursor
    }

    /// Delete the current character.
    fn on_delete(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        use buf::BackspaceRes::{Normal, PrevLine};
        match hq.buf(&self.buffer_name)?.backspace(self.spaces_after_cursor()) {
            Normal(mut after_cursor) => {
                after_cursor.push(' ');
                self.cursor -= 1;
                let line = Line::new_from_str(&after_cursor,
                                              self.view.theme.editor_cur());
                self.response_cursor_with_line(line, true)
            }
            PrevLine(cursor) => {
                // TODO: Implement this.
                self.cursor = cursor.x;
                //self.refresh(hq);
                Ok(Response::Unhandled)
            }
            _ => Ok(Response::Unhandled),
        }
    }

    /// Delete every characters after cursor.
    fn on_kill_line(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        use buf::KillLineRes::{Normal, Empty};
        match hq.buf(&self.buffer_name)?.kill_line() {
            Normal => {
                let line = Line::new_from_str(&vec![' '; self.spaces_after_cursor()]
                                                  .into_iter()
                                                  .collect::<String>(),
                                              self.view.theme.editor);
                self.response_cursor_with_line(line, false)
            }
            Empty(cursor) => {
                self.cursor = cursor.x;
                // TODO: Implement this.
                Ok(Response::Unhandled)
            }
            _ => Ok(Response::Unhandled),
        }
    }

    /// Update each of `line_cache`.
    pub fn resize(&mut self, _: &mut Hq, y: usize, width: usize) -> ResultBox<()> {
        self.view.y = y;
        self.view.width = width;
        Ok(())
    }

    /// Handle move events.
    fn on_move(&mut self, hq: &mut Hq, dx: i8, dy: i8) -> ResultBox<Response> {
        let (cursor_prev, cursor_now) = {
            let buf = hq.buf(&self.buffer_name)?;
            (buf.get_cursor(), buf.move_cursor(dx, dy))
        };
        if cursor_prev.y == cursor_now.y {
            // Move only in here.
            self.cursor = cursor_now.x;
            self.response_cursor()
        } else {
            // Pass the process to the parrent.
            panic!("H");
        }
    }

    /// Response wrapper for UI
    #[inline]
    fn response_ui(&self, response: res::Response) -> ResultBox<Response> {
        Ok(Response::Ui(response))
    }

    /// Response with cursor.
    fn response_cursor(&self) -> ResultBox<Response> {
        self.response_ui(
            res::Response::Term {
                cursor: Some(Cursor { x: self.cursor_translated(), y: 0 }),
                refresh: None,
            }
        )
    }

    /// Response with current cursor and following line.
    fn response_cursor_with_line(&self, line: Line, on_delete: bool)
                                 -> ResultBox<Response> {
        let x = self.cursor_translated();
        self.response_ui(
            res::Response::Term {
                refresh: Some(Refresh {
                    x: if on_delete { x } else { x - 1 },
                    y: 0,
                    rect: Rect::new_from_line(line),
                }),
                cursor: Some(Cursor {
                    x: x,
                    y: 0
                })
            }
        )
    }

    /// Move cursor left and right, or Type a character.
    pub fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        match k {
            event::Key::Ctrl('a') |
            event::Key::Home => {
                self.cursor = hq.buf(&self.buffer_name)?.move_begin_of_line().x;
                self.response_cursor()
            }
            event::Key::Ctrl('e') |
            event::Key::End => {
                self.cursor = hq.buf(&self.buffer_name)?.move_end_of_line().x;
                self.response_cursor()
            }
            event::Key::CR => {
                self.cursor = hq.buf(&self.buffer_name)?.break_line().x;
                // TODO: Implement this
                Ok(Response::LineBreak)
            }
            event::Key::Del => self.on_delete(hq),
            event::Key::Ctrl('k') => self.on_kill_line(hq),
            event::Key::Ctrl('n') |
            event::Key::Down => self.on_move(hq, 0, 1),
            event::Key::Ctrl('p') |
            event::Key::Up => self.on_move(hq, 0, -1),
            event::Key::Ctrl('f') |
            event::Key::Right => self.on_move(hq, 1, 0),
            event::Key::Ctrl('b') |
            event::Key::Left => self.on_move(hq, -1, 0),
            event::Key::Char(c) => {
                let mut after_cursor = String::with_capacity(self.view.width);
                after_cursor.push(c);
                after_cursor.push_str(&hq.buf(&self.buffer_name)?
                    .insert(c, self.spaces_after_cursor()));
                let line = Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
                self.cursor += 1;
                self.response_cursor_with_line(line, false)
            }
            _ => Ok(Response::Unhandled)
        }
    }
}
