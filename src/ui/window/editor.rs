use msg::event;
use hq::Hq;
use util::ResultBox;
use buf::{BackspaceRes, KillLineRes};
use ui::res::{Cursor, Line, Rect, Response, Refresh};
use ui::comp::{Component, ViewT};
use ui::window::LineEditor;

#[derive(Default, UiView)]
pub struct Editor {
    view: ViewT,
    line_editor: LineEditor,
    buffer_name: String,
    cursor: Cursor,
    line_max: usize,
    line_offset: usize,
    line_cache: Vec<Line>,
}

impl Editor {
    /// Update the line_caches.
    /// TODO: Reuse line_cache (expand, shrink).
    fn refresh_line_cache(&mut self, hq: &mut Hq) {
        let buf = hq.buf(&self.buffer_name).unwrap();
        let mut lc = self.line_offset;
        self.line_cache.clear();
        while let Some(_) = buf.get(lc) {
            let mut cache = Line::new_splitted(self.view.width,
                                               self.view.theme.linenum,
                                               self.view.theme.editor,
                                               self.linenum_width());
            cache.render_buf(buf, lc);
            lc += 1;
            self.line_cache.push(cache);
            if lc > self.view.height + self.line_offset {
                break;
            }
        }
    }

    #[inline]
    fn spaces_after_cursor(&self) -> usize {
        self.view.width - self.linenum_width() - self.cursor.x
    }

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

    fn cursor_translated(&self) -> Cursor {
        let mut cur = self.cursor;
        cur.x += self.linenum_width();
        cur
    }

    /// Calculate the screen's coordinate of the cursor.
    #[inline]
    fn translate_cursor(&self) -> Cursor {
        let line_idx = self.cursor.y - self.line_offset;
        Cursor {
            x: self.cursor.x + self.linenum_width(),
            y: line_idx,
        }
    }

    /// Basic initializennr.
    pub fn new() -> Editor {
        Editor {
            buffer_name: String::from("<empty>"),
            line_editor: LineEditor::new("<empty>"),
            ..Default::default()
        }
    }

    #[inline]
    fn resp_cursor(&self) -> ResultBox<Response> {
        Ok(Response::Term {
            cursor: Some(self.translate_cursor()),
            refresh: None,
        })
    }

    /// Handle move events.
    fn on_move(&mut self, hq: &mut Hq, dx: i8, dy: i8) -> ResultBox<Response> {
        let line_prev = self.cursor.y - self.line_offset;
        self.cursor = hq.buf(&self.buffer_name)?.move_cursor(dx, dy);
        if self.cursor.y < self.line_offset {
            // Scroll upward
            self.line_offset = self.cursor.y;
            self.refresh(hq)
        } else {
            let mut res = None;
            while self.translate_cursor().y >= self.view.height {
                // Scroll downward
                self.line_offset += 1;
                res = Some(self.refresh(hq)?);
            }
            if let Some(r) = res {
                Ok(r)
            } else {
                // Do not scroll
                let line_now = self.cursor.y - self.line_offset;
                match line_now {
                    _ if line_now < line_prev => {
                        // Upward
                        let y_off = line_now;
                        let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
                        rect.append(&self.line_editor.render(hq, y_off)?);
                        rect.append(&self.line_cache[line_prev]);
                        Ok(Response::Term {
                            refresh: Some(Refresh {
                                x: 0,
                                y: y_off,
                                rect: rect,
                            }),
                            cursor: Some(self.translate_cursor()),
                        })
                    }
                    _ if line_now > line_prev => {
                        // Downward
                        let y_off = line_prev;
                        let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
                        rect.append(&self.line_cache[line_prev]);
                        rect.append(&self.line_editor.render(hq, y_off)?);
                        Ok(Response::Term {
                            refresh: Some(Refresh {
                                x: 0,
                                y: y_off,
                                rect: rect,
                            }),
                            cursor: Some(self.translate_cursor()),
                        })
                    }
                    _ => self.resp_cursor(),
                }
            }
        }
    }
}

impl Component for Editor {
    /// Update each of `line_cache`.
    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()> {
        self.line_editor.resize(hq, 0, self.view.width)?;
        Ok(())
    }

    fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        use ui::window::line_editor::Response::*;
        match self.line_editor.on_key(hq, k)? {
            Ui(resp) => {
                Ok(resp)
            }
            LineBreak => {
                // Break the line.
                Ok(Default::default())
            }
            Unhandled => {
                Ok(Default::default())
            }
        }
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        {
            let linenum_width = self.linenum_width();
            self.line_editor.set_linenum_width(linenum_width);
        }
        self.refresh_line_cache(hq);
        let mut rect = Rect::new(self.view.width, 0, self.view.theme.linenum);
        for (i, line) in self.line_cache.iter().enumerate() {
            if i == self.cursor.y {
                rect.append(&self.line_editor.render(hq, i).unwrap());
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
            cursor: Some(self.translate_cursor()),
        })
    }

    /// Handle events.
    fn handle(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        match e {
            event::Event::OpenBuffer(s) => {
                self.line_editor.set_buffer_name(&s);
                self.buffer_name = s;
                self.line_max = hq.buf(&self.buffer_name)?.get_line_num();
                Ok(Default::default())
            }
            _ => Ok(Response::Unhandled),
        }
    }
}
