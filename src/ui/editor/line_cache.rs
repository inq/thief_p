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
        let mut linenum = self.y_offset;
        self.lines.clear();
        while let Some(_) = buffer.get(linenum) {
            self.refresh(view, buffer, linenum);
            linenum += 1;
            if linenum > view.height + self.y_offset {
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
        self.refresh(view, buffer, linenum - y_offset);
        &self.lines[linenum - self.y_offset]
    }

    /// Refresh from the buffer.
    /// Return true iff there is the correcsponding line.
    fn refresh(&mut self, view: &View, buffer: &mut buf::Buffer, linenum: usize) -> bool {
        // TODO: Reuse
        while self.lines.len() <= linenum {
            self.lines.push(term::Line::new_splitted(
                view.width,
                view.theme.linenum,
                view.theme.editor,
                self.linenum_width,
            ));
        }
        self.lines[linenum].draw_str_ex(
            &format!("{:width$}", linenum, width = self.linenum_width),
            0,
            0,
            view.theme.editor.fg,
            view.theme.arrow_fg,
        );
        if let Some(s) = buffer.get(linenum) {
            self.lines[linenum].draw_str_ex(
                s,
                self.linenum_width,
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
