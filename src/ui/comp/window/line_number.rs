use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component};

#[allow(dead_code)]
pub struct LineNumber {
    current: usize,
    max: usize,
    width: usize,
    height: usize,
}

impl Component for LineNumber {
    fn resize(&mut self, _: usize, height: usize) -> (usize, usize) {
        self.height = height;
        (self.width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(220, 180, 180));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        for (i, line) in buffer.lines.iter_mut().enumerate() {
            line.draw_str(&format!("{:width$}", i, width = self.width - 1), 0);
        }
        Response::refresh(0, 0, buffer)
    }
}

impl LineNumber {
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.width = format!("{}", self.max).len() + 1;
    }

    pub fn new() -> LineNumber {
        LineNumber {
            current: usize::max_value(),
            max: usize::max_value(),
            width: usize::max_value(),
            height: usize::max_value(),
        }
    }
}
