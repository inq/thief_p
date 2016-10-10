use ui::command_bar::CommandBar;
use ui::res::{Buffer, Brush, Color, Response};
use ui::window::Window;
use ui::comp::{Parent, Child, Component};

pub struct Screen {
    windows: Vec<Child>,
    command_bar: Child,
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
        self.command_bar.x = 0;
        self.command_bar.y = self.height - 1;
        self.command_bar.comp.resize(self.width, 3);
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut a = self.refresh_children(&mut buffer);
        let mut res = vec![Response::Refresh(buffer)];
        res.append(&mut a);
        res
    }
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let mut res = Screen {
            windows: vec![
                Child { comp: Box::new(Window::new(0, 0)), x: 0, y: 0 },
                Child { comp: Box::new(Window::new(0, 0)), x: 0, y: 0 },
            ],
            command_bar: Child { comp: Box::new(CommandBar::new(0, 0)), x: 0, y: 0 },
            width: 0,
            height: 0,
        };
        res.resize(width, height);
        res
    }
}

impl Parent for Screen {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        self.windows.iter_mut()
            .chain(Some(&mut self.command_bar).into_iter())
            .collect()
    }

    fn children(&self) -> Vec<&Child> {
        self.windows.iter()
            .chain(Some(&self.command_bar).into_iter())
            .collect()
    }
}
