use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, View};

#[derive(Default)]
pub struct LineNumber {
    pub current: usize,
    view: View,
    max: usize,
}

impl Component for LineNumber {
    fn get_view(&self) -> &View {
        &self.view
    }

    fn resize(&mut self, x: usize, y: usize, _: usize, height: usize) -> (usize, usize) {
        self.view.x = x;
        self.view.y = y;
        self.view.height = height;
        (self.view.width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(220, 180, 180));
        let mut buffer = Buffer::blank(&b, self.view.width, self.view.height);
        for (i, line) in buffer.lines.iter_mut().enumerate() {
            line.draw_str(
                &format!(
                    "{:width$}",
                    i + self.current,
                    width = self.view.width - 1),
                0);
        }
        Response::refresh(0, 0, buffer)
    }
}

impl LineNumber {
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.view.width = format!("{}", self.max).len() + 1;
    }
}
