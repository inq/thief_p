use ui::res::color::Brush;
use ui::res::line::Line;

#[allow(dead_code)]
pub struct Buffer {
    lines: Vec<Line>,
    width: usize,
    height: usize,
}

impl Buffer {
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

    #[allow(dead_code)]
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

    pub fn print(&self, mut buf: &mut String, brush: &Brush) -> Brush {
        let mut cur = brush.clone();
        for (i, l) in self.lines.iter().enumerate() {
            cur = l.print(&mut buf, &cur);
        }
        buf.push_str("\u{1b}[0m");
        cur
    }
}
