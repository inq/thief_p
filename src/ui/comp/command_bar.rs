use ui::res::{Buffer, Brush, Color, Cursor, Response, Refresh, Sequence};
use ui::comp::{Child, Component};

#[derive(Default)]
pub struct CommandBar {
    width: usize,
    height: usize,
}

impl Component for CommandBar {
    fn resize(&mut self, width: usize, _: usize) -> (usize, usize) {
        self.width = width;
        self.height = 1;
        (width, 1)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(220, 220, 220), Color::new(60, 30, 30));
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                buf: Buffer::blank(&b, self.width, self.height)
            }),
            sequence: vec![
                Sequence::Move(Cursor { x: 0, y: 0 }),
            ]
        }
    }
}

impl CommandBar {
    pub fn new() -> Child {
        let cb: CommandBar = Default::default();
        Child::new(Box::new(cb))
    }
}
