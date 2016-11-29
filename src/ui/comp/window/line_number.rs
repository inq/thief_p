use hq::Hq;
use util::ResultBox;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, View};

#[derive(Default)]
pub struct LineNumber {
    pub current: usize,
    view: View,
    max: usize,
}

impl Component for LineNumber {
    has_view!();

    /// Ignore width & force the width to be its own size.
    fn on_resize(&mut self) {
        self.update_width();
    }

    /// Draw the line numbers.
    fn refresh(&self, _: &mut Hq) -> ResultBox<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(220, 180, 180));
        let mut buffer = Buffer::blank(&b, self.view.width, self.view.height);
        for (i, line) in buffer.lines.iter_mut().enumerate() {
            line.draw_str(&format!("{:width$}", i + self.current, width = self.view.width - 1),
                          0);
        }
        Ok(Response::refresh(0, 0, buffer))
    }
}

impl LineNumber {
    #[inline]
    fn update_width(&mut self) {
        self.view.width = format!("{}", self.max).len() + 1;
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.update_width();
    }
}
