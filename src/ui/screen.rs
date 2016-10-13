use ui::command_bar::CommandBar;
use ui::res::{Buffer, Brush, Color, Response};
use ui::window::Window;
use ui::comp::{Parent, Child, Component};

pub struct Screen {
    window: Child,
    overlaps: Vec<Child>,
    width: usize,
    height: usize,
}

impl Component for Screen {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.window.comp.resize(self.width - 2, self.height - 2);
        self.window.x = 1;
        self.window.y = 1;
        for &mut ref mut child in self.overlaps.iter_mut() {
            child.comp.resize(self.width, 3);
            child.x = 0;
            child.y = self.height - 1;
        }
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut a = self.refresh_children(&mut buffer);
        let mut res = vec![Response::Refresh(0, 0, buffer)];
        res.append(&mut a);
        res
    }
}

impl Screen {
    pub fn command_bar(&mut self) -> Vec<Response> {
        if self.overlaps.len() == 0 {
            let bar = CommandBar::new(0, self.height - 1, self.width, 1);
            let res = bar.refresh();
            self.overlaps.push(bar);
            res
        } else {
            vec![]
        }
    }

    pub fn new(width: usize, height: usize) -> Screen {
        let mut res = Screen {
            window: Child { comp: Box::new(Window::new(0, 0)), x: 0, y: 0 },
            overlaps: vec![],
            width: 0,
            height: 0,
        };
        res.resize(width, height);
        res
    }

    pub fn key(&mut self, c: char, ctrl: bool) -> Vec<Response> {
        if ctrl {
            match c {
                'r' => self.command_bar(),
                _ => vec![]
            }
        } else {
            match c {
                'b' => vec![],
                _ => vec![],
            }
        }
    }
}

impl Parent for Screen {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        vec![&mut self.window].into_iter()
            .chain(self.overlaps.iter_mut())
            .collect()
    }

    fn children(&self) -> Vec<&Child> {
        vec![&self.window].into_iter()
            .chain(self.overlaps.iter())
            .collect()
    }
}
