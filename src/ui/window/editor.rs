use hq::Hq;
use io::Event;
use util::ResultBox;
use buf::{BackspaceRes, KillLineRes};
use ui::res::{Brush, Color, Cursor, Line, TextRect, Rect, Response, Refresh, Sequence};
use ui::comp::{Component, View};

#[derive(Default)]
pub struct Editor {
    view: View,
    buffer_name: String,
    cursor: Cursor,
    line_max: usize,
    line_offset: usize,
    line_cache: Vec<TextRect>,
    brush_l: Brush,
    brush_r: Brush,
}

impl Editor {
    fn render_lines(&mut self, hq: &mut Hq) {
        let buf = hq.buf(&self.buffer_name).unwrap();
        let mut lc = self.line_offset;
        let mut h = 0;
        self.line_cache.clear();
        while let Some(line) = buf.get(lc) {
            let mut cache = TextRect::new(self.view.width,
                                          self.brush_l,
                                          self.brush_r,
                                          self.line_num_width());
            cache.draw_line(&buf, lc);
            h += cache.height();
            lc += 1;
            self.line_cache.push(cache);
            // if h > self.view.height { break; }
        }
    }

    #[inline]
    fn spaces_after_cursor(&self) -> usize {
        self.view.width - self.line_num_width() - self.cursor.x
    }

    #[inline]
    fn line_num_width(&self) -> usize {
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
        let mut cur = self.cursor.clone();
        cur.x += self.line_num_width();
        cur
    }

    fn move_cursor(&self) -> Sequence {
        Sequence::Move(Cursor {
            x: self.cursor.x + self.line_num_width(),
            y: self.cursor.y - self.line_offset,
        })
    }

    /// Basic initializer.
    pub fn new() -> Editor {
        Editor {
            buffer_name: String::from("<empty>"),
            brush_l: Brush::new(Color::new(200, 200, 200), Color::new(100, 100, 100)),
            brush_r: Brush::new(Color::new(200, 200, 200), Color::new(40, 40, 40)),
            ..Default::default()
        }
    }
}

impl Component for Editor {
    has_view!();

    /// Update each of `line_cache`.
    fn on_resize(&mut self, _: &mut Hq) -> ResultBox<()> {
        Ok(())
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        self.render_lines(hq);
        let mut rect = Rect::new(self.view.width, 0, self.brush_l);
        for line in self.line_cache.iter() {
            if rect.append(&line, self.view.height).is_some() {
;
            } else {
                break;
            }
        }
        Ok(Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            sequence: vec![Sequence::Show(true), self.move_cursor()],
        })
    }

    /// Move cursor left and right, or Type a character.
    fn handle(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        match e {
            Event::OpenBuffer { s } => {
                self.buffer_name = s;
                self.line_max = hq.buf(&self.buffer_name)?.get_line_num();
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
                if self.cursor.y < self.line_offset {
                    // Scroll upward
                    self.line_offset = self.cursor.y;
                    self.refresh(hq)
                } else if self.cursor.y - self.line_offset >= self.view.height {
                    // Scroll downward
                    self.line_offset = self.cursor.y - self.view.height + 1;
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
                                                                             self.brush_r)),
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
                                                                             self.brush_r)),
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
                                   Sequence::Line(Line::new_from_str(&after_cursor, self.brush_r)),
                                   Sequence::Move(self.cursor_translated()),
                                   Sequence::Show(true)],
                    ..Default::default()
                })
            }
            _ => Ok(Default::default()),
        }
    }
}
