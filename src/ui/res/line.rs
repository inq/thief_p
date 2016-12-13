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

    /// Initialize a new line with two color brushes.
    /// The width of the left side is given by `splitter`.
    #[inline]
    pub fn new_splitted(width: usize, brush_l: Brush, brush_r: Brush, splitter: usize) -> Line {
        Line {
            chars: {
                let mut res = vec![Char::new(' ', brush_l); splitter];
                let mut tmp = vec![Char::new(' ', brush_r); width - splitter];
                res.append(&mut tmp);
                res
            },
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
    pub fn draw_buffer(&mut self, src: &buf::Line, linenum: usize, linenum_width: usize) {
        self.draw_str(&format!("{:width$}", linenum, width = linenum_width), 0);
        for (i, c) in src.iter().enumerate() {
            self.chars[i + linenum_width].chr = c;
        }
    }
}
