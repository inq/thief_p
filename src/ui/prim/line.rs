use ui::prim::char::Char;
use ui::prim::color::Brush;

#[derive(Debug, Clone)]
pub struct Line {
    chars: Vec<Char>,
    width: usize,
}

impl Line {
    pub fn bordered(line: &Brush, fill: &Brush, width: usize) -> Line {
        let mut chars = vec![Char{ chr: ' ', brush: line.clone()}; 1];
        chars.resize(width - 1,
                     Char {
                         chr: ' ',
                         brush: fill.clone(),
                     });
        chars.resize(width,
                     Char {
                         chr: ' ',
                         brush: line.clone(),
                     });
        Line {
            chars: chars,
            width: width,
        }
    }

    pub fn blank(brush: &Brush, width: usize) -> Line {
        Line {
            chars: vec![Char{ chr: ' ', brush: brush.clone() }; width],
            width: width,
        }
    }

    pub fn draw(&mut self, src: &Line, x: usize) {
        for (i, chr) in src.chars.iter().enumerate() {
            if x + i < self.width {
                self.chars[x + i] = src.chars[i].clone();
            }
        }
    }

    pub fn print(&self, mut buf: &mut String, brush: &Brush) -> Brush {
        let mut cur = brush.clone();
        for c in &self.chars {
            if c.brush != cur {
                cur.change(&mut buf, &c.brush);
                cur = c.brush.clone();
            }
            buf.push_str(&format!("{}", c.chr));
        }
        cur
    }
}
