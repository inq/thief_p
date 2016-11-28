use buf;
use ui::res::{Brush, Char, Formatted};

#[derive(Debug, Clone)]
pub struct Line {
    pub chars: Vec<Char>,
    pub width: usize,
}

impl Line {
    /// Construct a new line from str.
    pub fn new_from_str(src: &str, br: &Brush) -> Line {
        let res: Vec<Char> = src.chars().map(|c| Char::new(c, br.clone())).collect();
        let w = res.iter().map(|c| c.width()).sum();
        Line {
            chars: res,
            width: w,
        }
    }

    #[allow(dead_code)]
    pub fn bordered(line: &Brush, fill: &Brush, width: usize) -> Line {
        let mut chars = vec![Char::new(' ', line.clone()); 1];
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
            chars: vec![Char::new(' ', brush.clone()); width],
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

    /// Draw the formatted string here.
    pub fn draw_formatted(&mut self, src: &Formatted, x: usize) {
        for i in 0..src.len() {
            if let Some((style, c)) = src.get(i) {
                self.chars[x + i].overwrite(style, c);
            }
        }
    }

    /// Draw the given string into heer.
    pub fn draw_str(&mut self, src: &str, x: usize) {
        for (i, c) in src.chars().enumerate() {
            self.chars[x + i].chr = c
        }
    }

    /// Draw the given line buffer into here.
    pub fn draw_buffer(&mut self, src: &buf::Line, x: usize) {
        for (i, c) in src.iter().enumerate() {
            self.chars[x + i].chr = c;
        }
    }
}
