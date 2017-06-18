mod line_cache;
mod scrollable;

use buf;
use hq;
use term;
use ui;
use ui::comp::{Component, ViewT};
use ui::line_editor::LineEditor;
use util::ResultBox;

use self::scrollable::Scrollable;
use self::line_cache::LineCache;

#[derive(Default, UiView)]
pub struct Editor {
    view: ViewT,
    line_editor: LineEditor,
    buffer_name: String,
    line_max: usize,
    y_offset: usize,
    line_cache: LineCache,
}

impl Scrollable for Editor {
    fn line_editor(&self) -> &LineEditor {
        &self.line_editor
    }

    fn y_offset(&self) -> usize {
        self.y_offset
    }

    fn height(&self) -> usize {
        self.view.height
    }

    fn set_y_offset(&mut self, value: usize) {
        self.y_offset = value;
    }

    fn refresh_with_buffer(&mut self, buffer: &mut buf::Buffer) -> ResultBox<ui::Response> {
        let linenum_width = self.linenum_width();
        self.line_cache.refresh_all(
            &self.view,
            buffer,
            linenum_width,
            self.y_offset,
        );
        let rect = self.line_cache.render_to_rect(
            buffer,
            &self.view,
            &mut self.line_editor,
            self.y_offset,
        );
        let cursor = buffer.cursor();

        Ok(ui::Response::Term {
            refresh: Some(term::Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            cursor: Some(self.translate_cursor(cursor)),
        })
    }
}

impl Editor {
    /// TODO: Cache this.
    #[inline]
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

    /// Basic initializennr.
    pub fn new() -> Editor {
        Editor {
            buffer_name: String::from("<empty>"),
            line_editor: LineEditor::new(),
            ..Default::default()
        }
    }

    /// Response with rect and cursor.
    fn response_rect_with_cursor(
        &self,
        rect: term::Rect,
        y_offset: usize,
        cursor: term::Cursor,
    ) -> ResultBox<ui::Response> {
        Ok(ui::Response::Term {
            refresh: Some(term::Refresh {
                x: 0,
                y: y_offset,
                rect: rect,
            }),
            cursor: Some(self.translate_cursor(cursor)),
        })
    }

    /// Move the cursor vertically.
    fn on_move(
        &mut self,
        buffer: &mut buf::Buffer,
        cursor_prev: term::Cursor,
        cursor: term::Cursor,
    ) -> ResultBox<ui::Response> {
        if self.scroll(buffer) {
            return self.refresh_with_buffer(buffer);
        }
        if cursor.1 < cursor_prev.1 {
            // Move upward
            let mut rect = term::Rect::new(self.view.width, 0, self.view.theme.linenum);
            let linenum_width = self.linenum_width();
            rect.append(&self.line_editor.render(buffer)?);
            rect.append(&self.line_cache.refresh_one(
                &self.view,
                buffer,
                linenum_width,
                cursor_prev.1,
                self.y_offset,
            ));
            return self.response_rect_with_cursor(rect, cursor.1 - self.y_offset, cursor);
        }
        if cursor.1 > cursor_prev.1 {
            // Move downward
            let mut rect = term::Rect::new(self.view.width, 0, self.view.theme.linenum);
            let linenum_width = self.linenum_width();
            rect.append(self.line_cache.refresh_one(
                &self.view,
                buffer,
                linenum_width,
                cursor_prev.1,
                self.y_offset,
            ));
            rect.append(&self.line_editor.render(buffer)?);
            return self.response_rect_with_cursor(rect, cursor_prev.1 - self.y_offset, cursor);
        }
        unreachable!();
    }

    /// Return the buffer of this editor.
    fn get_buffer<'a>(&self, workspace: &'a mut hq::Workspace) -> ResultBox<&'a mut buf::Buffer> {
        workspace.buf(&self.buffer_name)
    }
}

impl Component for Editor {
    /// Update each of `line_cache`.
    fn on_resize(&mut self, _: &mut hq::Workspace) -> ResultBox<()> {
        self.line_editor.resize(0, self.view.width)?;
        Ok(())
    }

    /// Process keyboard event.
    fn on_key(&mut self, workspace: &mut hq::Workspace, k: term::Key) -> ResultBox<ui::Response> {
        use ui::line_editor::LineEditorRes::*;
        let buffer = self.get_buffer(workspace)?;
        match self.line_editor.on_key(buffer, k)? {
            Ui(resp) => {
                // TODO: Do something
                let y = buffer.y();
                Ok(resp.translate(0, y))
            }
            Move(p, c) => self.on_move(buffer, p, c),
            PullUp | LineBreak(_) | Refresh => {
                // TODO: Implement pull-up / pull-down instead of refresh
                Ok(self.refresh_with_buffer(buffer)?)
            }
            Unhandled => Ok(ui::Response::None),
        }
    }

    /// Refresh the editor.
    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response> {
        let linenum_width = self.linenum_width();
        self.line_editor.set_linenum_width(linenum_width);
        let buffer = self.get_buffer(workspace)?;
        self.refresh_with_buffer(buffer)
    }

    /// Handle events.
    fn handle(
        &mut self,
        workspace: &mut hq::Workspace,
        e: ::ui::Request,
    ) -> ResultBox<ui::Response> {
        match e {
            ::ui::Request::OpenBuffer(s) => {
                self.buffer_name = s;
                let buf = self.get_buffer(workspace)?;
                self.line_max = buf.line_num();
                Ok(ui::Response::None)
            }
            _ => Ok(ui::Response::Unhandled),
        }
    }
}
