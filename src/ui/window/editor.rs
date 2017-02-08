use msg::event;
use buf::Buffer;
use hq::Hq;
use util::ResultBox;
use ui::res::{Cursor, Line, Rect, Response, Refresh};
use ui::comp::{Component, ViewT};
use ui::window::LineEditor;

#[derive(Default, UiView)]
pub struct Editor {
    view: ViewT,
    line_editor: LineEditor,
    buffer_name: String,
    line_max: usize,
    y_offset: usize,
    line_cache: Vec<Line>,
}

impl Editor {
    #[inline]
    /// TODO: Cache this.
    fn linenum_width(&self) -> usize {
        let mut t = self.line_max;
        if t == 0 {
            2
        } else {
            let mut c = 0;
            while t > 0 {
                t /= 10;
                c += 1;
            }
            c + 1
        }
    }

    /// Calculate the screen's coordinate of the cursor.
    #[inline]
    fn translate_cursor(&self, cursor: Cursor) -> Cursor {
        // TODO: Apply the x_offset from line_editor.
        Cursor {
            x: cursor.x + self.linenum_width(),
            y: cursor.y - self.y_offset,
        }
    }

    /// Basic initializennr.
    pub fn new() -> Editor {
        Editor {
            buffer_name: String::from("<empty>"),
            line_editor: LineEditor::new(),
            ..Default::default()
        }
    }

    /// Response with rect and cursor.
    fn response_rect_with_cursor(&self,
                                 rect: Rect,
                                 y_offset: usize,
                                 cursor: Cursor)
                                 -> ResultBox<Response> {
        Ok(Response::Term {
            refresh: Some(Refresh {
                x: 0,
                y: y_offset,
                rect: rect,
            }),
            cursor: Some(self.translate_cursor(cursor)),
        })
    }

    /// Move the cursor vertically.
    fn on_move(&mut self, hq: &mut Hq, cursor_prev: Cursor, cursor: Cursor) -> ResultBox<Response> {
        if cursor.y < self.y_offset {
            // Scroll upward
            self.y_offset = cursor.y;
            return self.refresh(hq);
        }
        if cursor.y >= self.y_offset + self.view.height {
            // Scroll downward
            self.y_offset = cursor.y + self.view.height;
            return self.refresh(hq);
        }
        let buf = self.get_buffer(hq)?;
        if cursor.y < cursor_prev.y {
            // Move upward
            let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
            rect.append(&self.line_editor.render(buf, cursor.y)?);
            {
                let line_cache = self.refresh_line_cache(buf, cursor_prev.y);
                let prev_line_cache = &mut self.line_cache[cursor_prev.y - self.y_offset];
                *prev_line_cache = line_cache;
                rect.append(prev_line_cache);
            }
            return self.response_rect_with_cursor(rect, cursor.y - self.y_offset, cursor);
        }
        if cursor.y > cursor_prev.y {
            // Move downward
            let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
            rect.append(&self.line_cache[cursor_prev.y - self.y_offset]);
            rect.append(&self.line_editor.render(buf, cursor.y)?);
            return self.response_rect_with_cursor(rect, cursor_prev.y - self.y_offset, cursor);
        }
        unreachable!();
    }

    /// Refresh the single line cache.
    fn refresh_line_cache(&mut self, buf: &mut Buffer, linenum: usize) -> Line {
        // TODO: Merge with line_editor
        let mut res = Line::new_splitted(self.view.width,
                                         self.view.theme.linenum,
                                         self.view.theme.editor,
                                         self.linenum_width());
        res.draw_str(&format!("{:width$}", linenum, width = self.linenum_width()), 0, 0);
        if let Some(s) = buf.get(linenum) {
            res.draw_str(s, self.linenum_width(), 0);
        }
        res
    }

    /// Update the line_caches.
    /// TODO: Reuse line_cache (expand, shrink).
    fn refresh_line_caches(&mut self, hq: &mut Hq) -> Cursor {
        let buf = self.get_buffer(hq).unwrap();
        let mut linenum = self.y_offset;
        self.line_cache.clear();
        while let Some(_) = buf.get(linenum) {
            let cache = self.refresh_line_cache(buf, linenum);
            self.line_cache.push(cache);
            linenum += 1;
            if linenum > self.view.height + self.y_offset {
                break;
            }
        }
        buf.get_cursor()
    }

    /// Return the buffer of this editor.
    fn get_buffer<'a>(&self, hq: &'a mut Hq) -> ResultBox<&'a mut Buffer> {
        hq.buf(&self.buffer_name)
    }
}

impl Component for Editor {
    /// Update each of `line_cache`.
    fn on_resize(&mut self, _: &mut Hq) -> ResultBox<()> {
        self.line_editor.resize(0, self.view.width)?;
        Ok(())
    }

    /// Process keyboard event.
    fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        use ui::window::line_editor::Response::*;
        let res = {
            let buf = self.get_buffer(hq)?;
            self.line_editor.on_key(buf, k)?
        };
        match res {
            Ui(resp) => {
                // TODO: Do something
                let buf = self.get_buffer(hq)?;
                let y = buf.get_y();
                Ok(resp.translate(0, y))
            }
            Move(p, c) => self.on_move(hq, p, c),
            PullUp(_y) => {
                // Pull-up and refresh the line-editor.
                // TODO: Implement pull-up instead of refresh
                Ok(self.refresh(hq)?)
            }
            LineBreak(_cursor) => {
                // Break the line.
                // TODO: Implement pull-down instead of refresh
                Ok(self.refresh(hq)?)
            }
            Unhandled => Ok(Default::default()),
        }
    }

    /// Refresh the editor.
    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let linenum_width = self.linenum_width();
        self.line_editor.set_linenum_width(linenum_width);
        let cursor = self.refresh_line_caches(hq);
        let buf = self.get_buffer(hq)?;
        let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
        for (i, line) in self.line_cache.iter().enumerate() {
            if i + self.y_offset == cursor.y {
                rect.append(&self.line_editor.render(buf, i).unwrap());
            } else {
                rect.append(line);
            }
        }
        Ok(Response::Term {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            cursor: Some(self.translate_cursor(cursor)),
        })
    }

    /// Handle events.
    fn handle(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        match e {
            event::Event::OpenBuffer(s) => {
                self.buffer_name = s;
                let buf = self.get_buffer(hq)?;
                self.line_max = buf.get_line_num();
                Ok(Default::default())
            }
            _ => Ok(Response::Unhandled),
        }
    }
}
