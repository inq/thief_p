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

    pub fn to_string(&self, mut brush: &mut Option<Brush>) -> String {
        let mut res = String::with_capacity(self.width * self.height * 2);
        for l in self.lines.iter() {
            res.push_str(&l.to_string(&mut brush));
        }
        res
    }
}
