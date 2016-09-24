use std::io::{self, Write};
use ui::char::Char;
use ui::color::Brush;

pub struct Line {
    chars: Vec<Char>,
    width: u32,
}

impl Line {
    pub fn blank(brush: &Brush, width: u32) -> Line {
        Line {
            chars: vec![Char{ chr: ' ', brush: brush.clone() }; width as usize],
            width: width,
        }
    }

    pub fn print(&self, brush: &Brush) -> Brush {
        let mut cur = brush.clone();
        for c in &self.chars {
            if c.brush != cur {
                cur.change(&c.brush);
                cur = c.brush.clone();
            }
            print!("{}", c.chr);
        }
        print!("\u{1b}[0m");
        io::stdout().flush();
        cur
    }
}
