use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::{Child, Component};

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
            Response::Refresh(0, 0, Buffer::blank(&b, self.width, self.height)),
            Response::Move(Cursor { x: 0, y: 0 }),
        ]
    }
}

impl CommandBar {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Child {
        Child {
            x: x, y: y,
            comp: Box::new(CommandBar {
                width: width,
                height: height,
            })
        }
    }
}