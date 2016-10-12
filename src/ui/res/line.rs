use ui::res::char::Char;
use ui::res::color::Brush;

#[derive(Debug, Clone)]
pub struct Line {
    pub chars: Vec<Char>,
    pub width: usize,
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
                self.chars[x + i] = chr.clone();
            }
        }
    }
}
