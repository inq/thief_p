use buf;
use ui::res::color::Brush;
use ui::res::line::Line;

#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    #[allow(dead_code)]
    pub fn bordered(line: &Brush, fill: &Brush, width: usize, height: usize) -> Buffer {
        let mut lines = vec![Line::blank(line, width); 1];
        lines.resize(height - 1, Line::bordered(line, fill, width));
        lines.resize(height, Line::blank(line, width));
        Buffer {
            lines: lines,
            width: width,
            height: height,
        }
    }

    pub fn blank(brush: &Brush, width: usize, height: usize) -> Buffer {
        Buffer {
            lines: vec![Line::blank(brush, width); height],
            width: width,
            height: height,
        }
    }

    pub fn draw(&mut self, src: &Buffer, x: usize, y: usize) {
        for (i, line) in src.lines.iter().enumerate() {
            if y + i < self.height {
                self.lines[y + i].draw(line, x);
            }
        }
    }

    /// Draw the text buffer here.
    pub fn draw_buffer(&mut self, src: &buf::Buffer, x: usize, y: usize, off: usize) {
        for i in 0..src.get_line_num() {
            if y + i < self.height {
                if let Some(line) = src.get(i + off) {
                    self.lines[y + i].draw_buffer(line, x)
                }
            }
        }
    }
}
