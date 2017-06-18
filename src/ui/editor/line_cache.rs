use buf;
use term;
use ui::comp::{Component, ViewT};
use ui::line_editor::LineEditor;

#[derive(Default)]
pub struct LineCache {
    lines: Vec<term::Line>,
}

impl LineCache {
    /// Render to the Rect object
    pub fn render_to_rect(
        &self,
        buffer: &mut buf::Buffer,
        view: &ViewT,
        line_editor: &mut LineEditor,
        y_offset: usize,
    ) -> term::Rect {
        let mut rect = term::Rect::new(view.width, 0, view.theme.linenum);
        let cursor = buffer.cursor();
        for (i, line) in self.lines.iter().enumerate() {
            if i + y_offset == cursor.1 {
                rect.append(&line_editor.render(buffer).unwrap());
            } else {
                rect.append(line);
            }
        }
        rect
    }

    /// Update the line_caches.
    /// TODO: Reuse line_cache (expand, shrink).
    pub fn refresh_all(
        &mut self,
        view: &ViewT,
        buffer: &mut buf::Buffer,
        linenum_width: usize,
        y_offset: usize,
    ) {
        let mut linenum = y_offset;
        self.lines.clear();
        while let Some(_) = buffer.get(linenum) {
            self.refresh(view, linenum_width, buffer, linenum);
            linenum += 1;
            if linenum > view.height + y_offset {
                break;
            }
        }
    }

    pub fn refresh_one(
        &mut self,
        view: &ViewT,
        buffer: &mut buf::Buffer,
        linenum_width: usize,
        linenum: usize,
        y_offset: usize,
    ) -> &term::Line {
        self.refresh(view, linenum_width, buffer, linenum - y_offset);
        &self.lines[linenum - y_offset]
    }

    /// Refresh from the buffer.
    /// Return true iff there is the correcsponding line.
    fn refresh(
        &mut self,
        view: &ViewT,
        linenum_width: usize,
        buffer: &mut buf::Buffer,
        linenum: usize,
    ) -> bool {
        // TODO: Reuse
        while self.lines.len() <= linenum {
            self.lines.push(term::Line::new_splitted(
                view.width,
                view.theme.linenum,
                view.theme.editor,
                linenum_width,
            ));
        }
        self.lines[linenum].draw_str_ex(
            &format!("{:width$}", linenum, width = linenum_width),
            0,
            0,
            view.theme.editor.fg,
            view.theme.arrow_fg,
        );
        if let Some(s) = buffer.get(linenum) {
            self.lines[linenum].draw_str_ex(
                s,
                linenum_width,
                0,
                view.theme.editor.fg,
                view.theme.arrow_fg,
            );
            true
        } else {
            false
        }
    }
}
