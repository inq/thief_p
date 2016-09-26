use std::error;
use ui::prim::{term, Buffer, Brush, Color};
use ui::window::Window;
use ui::comp::{Parent, Child, Component, Cursor, Response};

pub struct Screen {
    windows: Vec<Child<Window>>,
    width: usize,
    height: usize,
}

impl Component for Screen {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let borders = self.windows.len() + 1;
        let windows = self.windows.len();
        let mut offset = 1;
        for (i, &mut ref mut child) in self.windows.iter_mut().enumerate() {
            let w = (self.width - borders + i + 1) / windows;
            child.comp.resize(w, self.height - 2);
            child.x = offset;
            child.y = 1;
            offset += w + 1;
        }
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        self.refresh_children(buffer)
    }
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let mut res = Screen {
            windows: vec![
                Child { comp: Window::new(0, 0), x: 0, y: 0 },
                Child { comp: Window::new(0, 0), x: 0, y: 0 },
            ],
            width: 0,
            height: 0,
        };
        res.resize(width, height);
        res
    }

}

impl Parent<Window> for Screen {
    fn children_mut(&mut self) -> Vec<&mut Child<Window>> {
        self.windows.iter_mut().collect()
    }

    fn children(&self) -> Vec<&Child<Window>> {
        self.windows.iter().collect()
    }
}
