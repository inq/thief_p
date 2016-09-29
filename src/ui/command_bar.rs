use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::Component;

pub struct CommandBar {
    width: usize,
    height: usize,
}

impl Component for CommandBar {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(220, 220, 220), Color::new(60, 30, 30));
        vec![
            Response::Refresh(Buffer::blank(&b, self.width, self.height)),
            Response::Move(Cursor { x: 0, y: 0 }),
        ]
    }
}

impl CommandBar {
    pub fn new(width: usize, height: usize) -> CommandBar {
        CommandBar {
            width: width,
            height: height,
        }
    }
}
