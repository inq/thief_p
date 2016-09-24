use std::io::{self, Write};
use ui::color::Brush;
use ui::line::Line;
use ui::term;

#[allow(dead_code)]
pub struct Buffer {
    lines: Vec<Line>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn blank(brush: &Brush, width: usize, height: usize) -> Buffer {
        Buffer {
            lines: vec![Line::blank(brush, width); height],
            width: width,
            height: height,
        }
    }

    pub fn print(&self, brush: &Brush) -> Brush {
        let mut cur = brush.clone();
        for (i, l) in self.lines.iter().enumerate() {
            term::movexy(0, i);
            cur = l.print(&cur);
            io::stdout().flush().unwrap();
        }
        print!("\u{1b}[0m");
        cur
    }
}
