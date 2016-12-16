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
}
