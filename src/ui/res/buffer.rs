use buf;
use ui::res::color::Brush;
use ui::res::line::Line;
use ui::res::formatted::Formatted;

#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    /// Initialize a new buffer with two color brushes.
    /// The width of the left side is given by `splitter`.
    pub fn new_splitted(width: usize, height: usize,
                        brush_l: Brush, brush_r: Brush,
                        splitter: usize) -> Buffer {
        Buffer {
            lines: vec![Line::new_splitted(width, brush_l, brush_r, splitter); height],
            width: width,
            height: height,
        }
    }

    pub fn blank(brush: &Brush, width: usize, height: usize) -> Buffer {
        Buffer {
            lines: vec![Line::blank(brush, width); height],
            width: width,
            height: height,
        }
    }

    pub fn draw(&mut self, src: &Buffer, x: usize, y: usize) {
        for (i, line) in src.lines.iter().enumerate() {
            if y + i < self.height {
                self.lines[y + i].draw(line, x);
            }
        }
    }

    /// Draw the formatted string here.
    pub fn draw_formatted(&mut self, src: &Formatted, x: usize, y: usize) {
        if let Some(ref mut line) = self.lines.get_mut(y) {
            line.draw_formatted(src, x)
        }
    }

    /// Draw the text buffer here.
    pub fn draw_buffer(&mut self, src: &buf::Buffer, start_from: usize, linenum_width: usize) {
        for i in 0..src.get_line_num() {
            if i < self.height {
                if let Some(line) = src.get(i + start_from) {
                    self.lines[i].draw_buffer(line, linenum_width)
                }
            }
        }
    }
}
