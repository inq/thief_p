use ui::res::{Buffer, Brush, Color, Cursor, Response, Refresh, Sequence};
use ui::comp::{Child, Component};

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
            refresh: Some(Refresh { x: 0, y: 0, buf: Buffer::blank(&b, self.width, self.height)}),
            sequence: vec![
                Sequence::Move(Cursor { x: 0, y: 0 }),
            ]
        }
    }
}

impl CommandBar {
    pub fn new() -> Child {
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(CommandBar {
                width: usize::max_value(),
                height: usize::max_value(),
            })
        }
    }
}
