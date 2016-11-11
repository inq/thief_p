use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Child, Parent};

pub struct LineNumber {
    current: usize,
    max: usize,
    width: usize,
    height: usize,
}

impl Component for LineNumber {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.width = width;
        self.height = height;
        (width, height)
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 200, 220));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        vec![
            Response::Refresh(
                0, 0,
                buffer,
            ),
        ]
    }
}

impl LineNumber {
    pub fn new() -> LineNumber {
        LineNumber {
            current: usize::max_value(),
            max: usize::max_value(),
            width: usize::max_value(),
            height: usize::max_value(),
        }
    }
}
