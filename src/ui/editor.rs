use ui::res::{Buffer, Brush, Color, Cursor, Response};
use ui::comp::Component;

pub struct Editor {
    width: usize,
    height: usize,
}

impl Component for Editor {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(240, 220, 220));
        vec![
            Response::Refresh(
                0, 0,
                Buffer::bordered(&b, &b.invert(), self.width, self.height)
            ),
            Response::Move(Cursor { x: 0, y: 0 }),
        ]
    }
}

impl Editor {
    pub fn new(width: usize, height: usize) -> Editor {
        Editor {
            width: width,
            height: height,
        }
    }
}
