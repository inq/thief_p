use ui;
use buf::Buffer;
use term;
use util::ResultBox;
use ui::comp::ViewT;

#[allow(dead_code)]
#[derive(Default)]
pub struct LineEditor {
    view: ViewT,
    linenum_width: usize,
    x_offset: usize,
    more_right: bool,
}

pub enum LineEditorRes {
    Ui(ui::Response),
    LineBreak(term::Cursor),
    PullUp(usize),
    Move(term::Cursor, term::Cursor),
    Refresh,
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
    ///
    /// TODO: Process long lines
    pub fn render(&mut self, buf: &mut Buffer, linenum: usize) -> ResultBox<term::Line> {
        let mut cache = term::Line::new_splitted(self.view.width,
                                                 self.view.theme.linenum_cur(),
                                                 self.view.theme.editor_cur(),
                                                 self.linenum_width);
        cache.draw_str(&format!("{:width$}", linenum, width = self.linenum_width),
                       0,
                       0);
        if let Some(s) = buf.get(linenum) {
            let margin = if self.x_offset > 0 {
                cache.draw_str_ex(&"<<",
                               self.linenum_width,
                               0,
                               self.view.theme.arrow_fg,
                               self.view.theme.arrow_fg);
                2
            } else {
                0
            };
            self.more_right = cache.draw_str_ex(&s[self.x_offset + margin..],
                                             self.linenum_width + margin,
                                             0,
                                             self.view.theme.editor.fg,
                                             self.view.theme.arrow_fg);
        }
        Ok(cache)
    }

    /// Calculate the screen's coordinate of the cursor.
    #[inline]
    pub fn translate_cursor(&self, cursor: usize) -> usize {
        assert!(cursor + self.linenum_width >= self.x_offset);
        cursor + self.linenum_width - self.x_offset
    }

    #[inline]
    fn spaces_after_cursor(&self, cursor: usize) -> usize {
        self.view.width - self.linenum_width - cursor
    }

    /// Delete the current character.
    fn on_delete(&mut self, buf: &mut Buffer) -> ResultBox<LineEditorRes> {
        use buf::BackspaceRes::{Normal, PrevLine};
        let cursor = buf.get_x();
        match buf.backspace(self.spaces_after_cursor(cursor)) {
            Normal(mut after_cursor) => {
                after_cursor.push(' ');
                let line = term::Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
                self.response_cursor_with_line(buf.get_x(), line, true)
            }
            PrevLine(_cursor) => {
                // TODO: Implement this.
                Ok(LineEditorRes::Unhandled)
            }
            _ => Ok(LineEditorRes::Unhandled),
        }
    }

    /// Delete every characters after cursor.
    fn on_kill_line(&mut self, buf: &mut Buffer) -> ResultBox<LineEditorRes> {
        use buf::KillLineRes::{Normal, Empty};
        let cursor = buf.get_x();
        match buf.kill_line() {
            Normal => {
                let line = term::Line::new_from_str(&vec![' '; self.spaces_after_cursor(cursor)]
                                                         .into_iter()
                                                         .collect::<String>(),
                                                    self.view.theme.editor);
                self.response_cursor_with_line(buf.get_x(), line, false)
            }
            Empty(cursor) => Ok(LineEditorRes::PullUp(cursor.1)),
            _ => Ok(LineEditorRes::Unhandled),
        }
    }

    /// Update the y and width.
    pub fn resize(&mut self, y: usize, width: usize) -> ResultBox<()> {
        self.view.y = y;
        self.view.width = width;
        Ok(())
    }

    fn width(&self) -> usize {
        self.view.width - self.linenum_width - if self.more_right { 2 } else { 0 }
    }

    /// Handle move events.
    fn on_move(&mut self, buf: &mut Buffer, dx: i8, dy: i8) -> ResultBox<LineEditorRes> {
        let cursor_prev = buf.get_cursor();
        let cursor = buf.move_cursor(dx, dy);
        if cursor_prev.1 == cursor.1 {
            // Move only in here.
            if cursor.0 - self.x_offset >= self.width() {
                // Scroll right
                self.x_offset += 1;
                Ok(LineEditorRes::Refresh)
            } else if self.x_offset > 0 && cursor.0 - self.x_offset <= 1 {
                self.x_offset -= 1;
                Ok(LineEditorRes::Refresh)
            } else {
                self.response_cursor(cursor.0)
            }
        } else {
            // Pass the process to the parrent.
            Ok(LineEditorRes::Move(cursor_prev, cursor))
        }
    }

    /// Response wrapper for UI
    #[inline]
    fn response_ui(&self, response: ui::Response) -> ResultBox<LineEditorRes> {
        Ok(LineEditorRes::Ui(response))
    }

    /// Response with cursor.
    fn response_cursor(&self, cursor: usize) -> ResultBox<LineEditorRes> {
        self.response_ui(ui::Response::Term {
                             cursor: Some((self.translate_cursor(cursor), 0)),
                             refresh: None,
                         })
    }

    /// Response with current cursor and following line.
    fn response_cursor_with_line(&self,
                                 cursor: usize,
                                 line: term::Line,
                                 on_delete: bool)
                                 -> ResultBox<LineEditorRes> {
        let x = self.translate_cursor(cursor);
        self.response_ui(ui::Response::Term {
                             refresh: Some(term::Refresh {
                                               x: if on_delete { x } else { x - 1 },
                                               y: 0,
                                               rect: term::Rect::new_from_line(line),
                                           }),
                             cursor: Some((x, 0)),
                         })
    }

    /// Accept the char input.
    pub fn on_char(&mut self, buf: &mut Buffer, c: char) -> ResultBox<LineEditorRes> {
        if self.view.width < buf.get_x() + 4 {
            panic!("{} {}", self.view.width, buf.get_x());
            let cursor = buf.get_x();
            let after_cursor = String::with_capacity(self.view.width);
            let line = term::Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
            self.response_cursor_with_line(cursor, line, false)
        } else {
            let mut after_cursor = String::with_capacity(self.view.width);
            after_cursor.push(c);
            let cursor = buf.get_x();
            after_cursor.push_str(&buf.insert(c, self.spaces_after_cursor(cursor)));
            let cursor = buf.get_x();
            let line = term::Line::new_from_str(&after_cursor, self.view.theme.editor_cur());
            self.response_cursor_with_line(cursor, line, false)
        }
    }

    /// Move cursor left and right, or Type a character.
    pub fn on_key(&mut self, buf: &mut Buffer, k: term::Key) -> ResultBox<LineEditorRes> {
        match k {
            term::Key::Ctrl('a') |
            term::Key::Home => {
                let cursor = buf.move_begin_of_line().0;
                self.response_cursor(cursor)
            }
            term::Key::Ctrl('e') |
            term::Key::End => {
                let cursor = buf.move_end_of_line().0;
                self.response_cursor(cursor)
            }
            term::Key::CR => {
                let cursor = buf.break_line();
                Ok(LineEditorRes::LineBreak(cursor))
            }
            term::Key::Del => self.on_delete(buf),
            term::Key::Ctrl('k') => self.on_kill_line(buf),
            term::Key::Ctrl('n') |
            term::Key::Down => self.on_move(buf, 0, 1),
            term::Key::Ctrl('p') |
            term::Key::Up => self.on_move(buf, 0, -1),
            term::Key::Ctrl('f') |
            term::Key::Right => self.on_move(buf, 1, 0),
            term::Key::Ctrl('b') |
            term::Key::Left => self.on_move(buf, -1, 0),
            term::Key::Char(c) => self.on_char(buf, c),
            _ => Ok(LineEditorRes::Unhandled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_key() {
        let mut editor = LineEditor::new();
        editor.resize(0, 10);
        let mut buffer = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!("[package] ",
                   format!("{}", editor.render(&mut buffer, 0).unwrap()));
        editor.on_char(&mut buffer, 'a').unwrap();
        assert_eq!("a[package]",
                   format!("{}", editor.render(&mut buffer, 0).unwrap()));
        editor.on_char(&mut buffer, 'a');
        assert_eq!("aa[packa>>",
                   format!("{}", editor.render(&mut buffer, 0).unwrap()));
        editor.on_char(&mut buffer, 'a');
        editor.on_char(&mut buffer, 'a');
        assert_eq!("aaaa[pac>>",
                   format!("{}", editor.render(&mut buffer, 0).unwrap()));
        editor.on_char(&mut buffer, 'a');
        editor.on_char(&mut buffer, 'a');
        editor.on_char(&mut buffer, 'a');
        editor.on_char(&mut buffer, 'a');
        assert_eq!("aaaaaaaa>>",
                   format!("{}", editor.render(&mut buffer, 0).unwrap()));
    }
}
