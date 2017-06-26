use buf;
use term;
use ui::comp::View;
use ui::line_editor::LineEditor;

#[derive(Default)]
pub struct LineCache {
    lines: Vec<term::Line>,
    linenum_max: usize,
    linenum_width: usize,
    y_offset: usize,
}

impl LineCache {
    pub fn y_offset(&self) -> usize {
        self.y_offset
    }

    pub fn set_y_offset(&mut self, value: usize) {
        self.y_offset = value;
    }

    fn calculate_linenum_width(linenum_max: usize) -> usize {
        let mut t = linenum_max;
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

    pub fn set_linenum_max(&mut self, value: usize) {
        self.linenum_max = value;
        self.linenum_width = LineCache::calculate_linenum_width(value);
    }

    pub fn linenum_width(&self) -> usize {
        self.linenum_width
    }

    /// Render to the Rect object
    pub fn render_to_rect(
        &self,
        buffer: &mut buf::Buffer,
        view: &View,
        line_editor: &mut LineEditor,
    ) -> term::Rect {
        let mut rect = term::Rect::new(view.width, 0, view.theme.linenum);
        let cursor = buffer.cursor();
        for (i, line) in self.lines.iter().enumerate() {
            if i + self.y_offset == cursor.1 {
                rect.append(&line_editor.render(buffer).unwrap());
            } else {
                rect.append(line);
            }
        }
        rect
    }

    /// Update the line_caches.
    /// TODO: Reuse line_cache (expand, shrink).
    pub fn refresh_all(&mut self, view: &View, buffer: &mut buf::Buffer) {
        self.lines.clear();

        let mut line_idx = 0;
        while let Some(_) = buffer.get(line_idx + self.y_offset) {
            self.refresh(view, buffer, line_idx);
            line_idx += 1;
            if line_idx > view.height {
                break;
            }
        }
    }

    pub fn refresh_one(
        &mut self,
        view: &View,
        buffer: &mut buf::Buffer,
        linenum: usize,
    ) -> &term::Line {
        let y_offset = self.y_offset;
        let line_idx = linenum - y_offset;
        self.refresh(view, buffer, line_idx);
        &self.lines[line_idx]
    }

    /// Refresh from the buffer.
    /// Return true iff there is the correcsponding line.
    fn refresh(&mut self, view: &View, buffer: &mut buf::Buffer, line_idx: usize) -> bool {
        // TODO: Reuse
        while self.lines.len() <= line_idx {
            self.lines.push(term::Line::new_splitted(
                view.width,
                view.theme.linenum,
                view.theme.editor,
                self.linenum_width,
            ));
        }
        let linenum = self.y_offset + line_idx;
        // TODO: Coloring
        self.lines[line_idx].draw_str_ex(
            &term::String::from_std(
                &format!("{:width$}", linenum, width = self.linenum_width),
                view.theme.editor,
            ),
            0,
            0,
            view.theme.arrow_fg,
        );
        if let Some(s) = buffer.get(linenum) {
            self.lines[line_idx].draw_str_ex(s, self.linenum_width, 0, view.theme.arrow_fg);
            true
        } else {
            false
        }
    }
}
