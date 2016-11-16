use ui::res::{Buffer, Brush, Color, Cursor, Response, Refresh, Sequence};
use ui::comp::{Component, View};

#[derive(Default)]
pub struct CommandBar {
    view: View,
}

impl Component for CommandBar {
    fn get_view(&self) -> &View {
        &self.view
    }

    fn resize(&mut self, x: usize, y: usize, width: usize, _: usize) -> (usize, usize) {
        self.view.x = x;
        self.view.y = y;
        self.view.width = width;
        self.view.height = 1;
        (width, 1)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(220, 220, 220), Color::new(60, 30, 30));
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                buf: Buffer::blank(&b, self.view.width, self.view.height)
            }),
            sequence: vec![
                Sequence::Move(Cursor { x: 0, y: 0 }),
            ]
        }
    }
}
