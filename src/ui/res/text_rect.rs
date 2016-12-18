use buf;
use ui::res::{Brush, Cursor, Line};

#[derive(Debug)]
pub struct TextRect {
    lines: Vec<Line>,
    splitter: usize,
    brush_l: Brush,
    brush_r: Brush,
}

impl TextRect {
    /// Initialize a new rectangular buffer with two color brushes.
    /// The width of the left side is given by `splitter`.
    pub fn new(width: usize, brush_l: Brush, brush_r: Brush, splitter: usize) -> TextRect {
        TextRect {
            lines: vec![Line::new_splitted(width, brush_l, brush_r, splitter)],
            splitter: splitter,
            brush_l: brush_l,
            brush_r: brush_r,
        }
    }

    /// Return the lines.
    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    /// Return the width of the object.
    pub fn width(&self) -> usize {
        self.lines[0].width
    }

    /// Return the height of the object.
    #[inline]
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Initialize a new empty line.
    #[inline]
    fn default_line(&self) -> Line {
        Line::new_splitted(self.width(), self.brush_l, self.brush_r, self.splitter)
    }

    /// Return the cursor position.
    pub fn cursor_position(&self, x: usize) -> Cursor {
        let mut acc = 0;
        // Skip the last line
        for (y, line) in self.lines.iter().take(self.height() - 1).enumerate() {
            if x - acc < line.text_width() {
                return Cursor {
                    x: x - acc + self.splitter,
                    y: y,
                };
            } else {
                acc += line.text_width();
            }
        }
        Cursor {
            x: x - acc + self.splitter,
            y: self.height() - 1,
        }
    }

    /// Fill with the given brushes.
    #[inline]
    pub fn fill_brush(&mut self, brush_l: Brush, brush_r: Brush) {
        for mut line in self.lines.iter_mut() {
            line.fill_splitted(brush_l, brush_r, self.splitter);
        }
    }

    /// Draw the text buffer here.
    pub fn draw_line(&mut self, buf: &buf::Buffer, line_num: usize) {
        let line = if let Some(l) = buf.get(line_num) {
            l
        } else {
            return;
        };
        let mut off = 0;
        let mut y = 0;
        while let Some(o) = self.lines[y].draw_buffer(line, off, line_num, self.splitter) {
            if self.lines.len() > y {
                let new_line = self.default_line();
                self.lines.push(new_line);
                off += o;
            }
            y += 1;
        }
    }
}
