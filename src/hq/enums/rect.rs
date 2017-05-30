use term;

#[derive(Debug)]
pub struct Rect {
    pub lines: Vec<term::Line>,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(width: usize, height: usize, brush: term::Brush) -> Rect {
        Rect {
            lines: vec![term::Line::new(width, brush); height],
            width: width,
            height: height,
        }
    }

    /// Create a new Rect from Line.
    pub fn new_from_line(line: term::Line) -> Rect {
        let w = line.width;
        Rect {
            lines: vec![line],
            width: w,
            height: 1,
        }
    }

    /// Create a new Rect from Char.
    pub fn new_from_char(char: term::Char) -> Rect {
        let line = term::Line::new_from_char(char);
        let w = line.width;
        Rect {
            lines: vec![line],
            width: w,
            height: 1,
        }
    }

    pub fn draw_str(&mut self, src: &str, x: usize, y: usize) {
        self.lines[y].draw_str(src, x, 0);
    }

    pub fn draw(&mut self, src: &Rect, x: usize, y: usize) {
        for (i, line) in src.lines.iter().enumerate() {
            if y + i < self.height {
                self.lines[y + i].draw(line, x);
            }
        }
    }

    /// Append a Line object.
    pub fn append(&mut self, src: &term::Line) {
        self.lines.push(src.clone());
        self.height += 1;
    }

    /// Draw the formatted string here.
    pub fn draw_formatted(&mut self, src: &term::Formatted, x: usize, y: usize) {
        if let Some(ref mut line) = self.lines.get_mut(y) {
            line.draw_formatted(src, x)
        }
    }
}
