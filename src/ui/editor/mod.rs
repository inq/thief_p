mod line_cache;
mod scrollable;

use buf;
use hq;
use term;
use ui;
use ui::comp::{Component, View};
use ui::line_editor::LineEditor;
use util::ResultBox;

use self::scrollable::Scrollable;
use self::line_cache::LineCache;

#[derive(Default, UiView)]
pub struct Editor {
    view: View,
    line_editor: LineEditor,
    buffer_name: String,
    line_cache: LineCache,
}

impl Scrollable for Editor {
    fn line_editor(&self) -> &LineEditor {
        &self.line_editor
    }

    fn y_offset(&self) -> usize {
        self.line_cache.y_offset()
    }

    fn height(&self) -> usize {
        self.view.height
    }

    fn set_y_offset(&mut self, value: usize) {
        self.line_cache.set_y_offset(value);
    }

    fn refresh_with_buffer(&mut self, buffer: &mut buf::Buffer) -> ResultBox<ui::Response> {
        self.line_cache.refresh_all(&self.view, buffer);
        let rect = self.line_cache.render_to_rect(
            buffer,
            &self.view,
            &mut self.line_editor,
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
    fn set_linenum_max(&mut self, value: usize) {
        self.line_cache.set_linenum_max(value);
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
            rect.append(&self.line_editor.render(buffer)?);
            rect.append(self.line_cache.refresh_one(
                &self.view,
                buffer,
                cursor_prev.1,
            ));
            return self.response_rect_with_cursor(
                rect,
                cursor.1 - self.line_cache.y_offset(),
                cursor,
            );
        }
        if cursor.1 > cursor_prev.1 {
            // Move downward
            let mut rect = term::Rect::new(self.view.width, 0, self.view.theme.linenum);
            rect.append(self.line_cache.refresh_one(
                &self.view,
                buffer,
                cursor_prev.1,
            ));
            rect.append(&self.line_editor.render(buffer)?);
            return self.response_rect_with_cursor(
                rect,
                cursor_prev.1 - self.line_cache.y_offset(),
                cursor,
            );
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
            PullUp | LineBreak(_) => {
                // TODO: Implement pull-up / pull-down instead of refresh
                self.set_linenum_max(buffer.line_num());
                Ok(self.refresh_with_buffer(buffer)?)
            }
            Refresh => Ok(self.refresh_with_buffer(buffer)?),
            Unhandled => Ok(ui::Response::None),
        }
    }

    /// Refresh the editor.
    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response> {
        self.line_editor.set_linenum_width(
            self.line_cache.linenum_width(),
        );
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
                let buffer = self.get_buffer(workspace)?;
                self.set_linenum_max(buffer.line_num());
                Ok(ui::Response::None)
            }
            _ => Ok(ui::Response::Unhandled),
        }
    }
}
