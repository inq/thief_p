use buf;
use ui::res::color::Brush;
use ui::res::line::Line;
use ui::res::formatted::Formatted;

#[derive(Debug)]
pub struct TextRect {
    lines: Vec<Line>,
    splitter: usize,
    brush_l: Brush,
    brush_r: Brush,
}

impl TextRect {
    /// Initialize a new rectangular buffer with two color brushes.
    /// The width of the left side is given by `splitter`.
    pub fn new(width: usize, brush_l: Brush, brush_r: Brush, splitter: usize) -> TextRect {
        TextRect {
            lines: vec![Line::new_splitted(width, brush_l, brush_r, splitter)],
            splitter: splitter,
            brush_l: brush_l,
            brush_r: brush_r,
        }
    }

    /// Return the lines.
    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    /// Return the width of the object.
    pub fn width(&self) -> usize {
        self.lines[0].width
    }

    /// Return the height of the object.
    #[inline]
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    #[inline]
    fn default_line(&self) -> Line {
        Line::new_splitted(self.width(), self.brush_l, self.brush_r, self.splitter)
    }

    /// Draw the text buffer here.
    pub fn draw_line(&mut self, buf: &buf::Buffer, line_num: usize) {
        if let Some(line) = buf.get(line_num) {
            let mut off = 0;
            let mut acc = 0;
            let y = 0;
            while let Some(o) = self.lines[y].draw_buffer(line, off, line_num, self.splitter) {
                if self.lines.len() > y {
                    let new_line = self.default_line();
                    self.lines.push(new_line);
                    off = o;
                }
            }
        }
    }
}
