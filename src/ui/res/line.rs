use ui::res::{Brush, Char, Formatted};
use buf::Buffer;

#[derive(Debug, Clone)]
pub struct Line {
    pub chars: Vec<Char>,
    pub width: usize,
    text_width: usize,
    splitter: usize,
}

impl Line {
    /// Construct a new line from str.
    pub fn new_from_str(src: &str, brush: Brush) -> Line {
        let res: Vec<Char> = src.chars().map(|c| Char::new(c, brush)).collect();
        let w = res.iter().map(|c| c.width()).sum();
        Line {
            chars: res,
            width: w,
            text_width: 0,
            splitter: 0,
        }
    }

    /// Construct a new line from Char.
    pub fn new_from_char(char: Char) -> Line {
        let w = char.width();
        Line {
            chars: vec![char],
            width: w,
            text_width: 1,
            splitter: 0,
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
            text_width: 0,
            splitter: splitter,
        }
    }

    #[inline]
    pub fn render_buf(&mut self, buf: &mut Buffer, linenum: usize) {
        let splitter = self.splitter;
        self.draw_str(&format!("{:width$}", linenum, width = splitter), 0);
        if let Some(ref s) = buf.get(linenum) {
            self.draw_str(&s, splitter);
        }
    }

    /// Fill with the given brushes.
    #[inline]
    pub fn fill_splitted(&mut self, brush_l: Brush, brush_r: Brush, splitter: usize) {
        for (i, c) in self.chars.iter_mut().enumerate() {
            if i < splitter {
                c.brush = brush_l;
            } else {
                c.brush = brush_r;
            }
        }
    }

    /// Initialize a new empty line.
    #[inline]
    pub fn new(width: usize, brush: Brush) -> Line {
        Line {
            chars: vec![Char::new(' ', brush); width],
            width: width,
            text_width: 0,
            splitter: 0,
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

    /// Return the actual text width.
    #[inline]
    pub fn text_width(&self) -> usize {
        self.text_width
    }

    /// Draw the given line buffer into here. If there is no space, return the remaining.
    #[inline]
    pub fn draw_buffer(&mut self,
                       src: &str,
                       offset: usize,
                       linenum: usize,
                       linenum_width: usize)
                       -> Option<usize> {
        if offset == 0 {
            // Draw the line number only if the offset is zero.
            self.draw_str(&format!("{:width$}", linenum, width = linenum_width), 0);
        }
        for (i, c) in src.chars().skip(offset).enumerate() {
            self.text_width = i;
            if i + linenum_width < self.width {
                self.chars[i + linenum_width].chr = c;
            } else {
                return Some(i);
            }
        }
        None
    }
}
