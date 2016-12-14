use buf;
use ui::res::{Formatted, Brush, Line, TextRect};

#[derive(Debug)]
pub struct Rect {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(width: usize, height: usize, brush: Brush) -> Rect {
        Rect {
            lines: vec![Line::new(width, brush); height],
            width: width,
            height: height,
        }
    }

    pub fn draw(&mut self, src: &Rect, x: usize, y: usize) {
        for (i, line) in src.lines.iter().enumerate() {
            if y + i < self.height {
                self.lines[y + i].draw(line, x);
            }
        }
    }

    /// Append a TextRect object.
    pub fn append(&mut self, src: &TextRect, limit: usize) -> Option<()> {
        for line in src.lines().clone() {
            if self.lines.len() < limit {
                self.lines.push(line)
            } else {
                return None;
            }
        }
        Some(())
    }

    /// Draw the formatted string here.
    pub fn draw_formatted(&mut self, src: &Formatted, x: usize, y: usize) {
        if let Some(ref mut line) = self.lines.get_mut(y) {
            line.draw_formatted(src, x)
        }
    }

    /// Draw the text buffer here.
    pub fn draw_buffer(&mut self, src: &buf::Buffer, start_from: usize, linenum_width: usize) {
        let mut cur = if let Some(line) = src.get(start_from) {
            line
        } else {
            return;
        };
        let mut off = 0;
        let mut acc = 0;
        for y in 0..self.height {
            if let Some(o) = self.lines[y].draw_buffer(cur, off, start_from + acc, linenum_width) {
                off = o;
            } else {
                off = 0;
                acc += 1;
                if let Some(line) = src.get(start_from + acc) {
                    cur = line;
                } else {
                    return;
                }
            }
        }
    }
}
