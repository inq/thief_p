use term;

#[derive(Debug, Clone)]
pub struct Line {
    pub chars: Vec<term::Char>,
    pub width: usize,
    text_width: usize,
    splitter: usize,
}

impl Line {
    /// Construct a new line from str.
    pub fn new_from_str(src: &str, brush: term::Brush) -> Line {
        let res: Vec<term::Char> = src.chars().map(|c| term::Char::new(c, brush)).collect();
        let w = res.iter().map(|c| c.width()).sum();
        Line {
            chars: res,
            width: w,
            text_width: 0,
            splitter: 0,
        }
    }

    /// Construct a new line from Char.
    pub fn new_from_char(char: term::Char) -> Line {
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
    pub fn new_splitted(width: usize,
                        brush_l: term::Brush,
                        brush_r: term::Brush,
                        splitter: usize)
                        -> Line {
        Line {
            chars: {
                let mut res = vec![term::Char::new(' ', brush_l); splitter];
                let mut tmp = vec![term::Char::new(' ', brush_r); width - splitter];
                res.append(&mut tmp);
                res
            },
            width: width,
            text_width: 0,
            splitter: splitter,
        }
    }

    /// Initialize a new empty line.
    #[inline]
    pub fn new(width: usize, brush: term::Brush) -> Line {
        Line {
            chars: vec![term::Char::new(' ', brush); width],
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
    pub fn draw_formatted(&mut self, src: &term::Formatted, x: usize) {
        for i in 0..src.len() {
            if let Some((style, c)) = src.get(i) {
                self.chars[x + i].overwrite(style, c);
            }
        }
    }

    /// Draw the given string into heer.
    pub fn draw_str(&mut self, src: &str, x: usize, limit: usize) {
        let limit_x = if limit == 0 { self.width - x } else { limit };
        for (i, c) in src.chars().enumerate().take(limit_x) {
            self.chars[x + i].chr = c
        }
    }
}
