use ui::res::{Buffer, Brush, Color, Response};
use ui::window::Window;
use ui::comp::{Parent, Child, Component};

pub struct HSplit {
    windows: Vec<Child>,
    width: usize,
    height: usize,
}

impl Component for HSplit {
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

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut a = self.refresh_children(&mut buffer);
        let mut res = vec![Response::Refresh(0, 0, buffer)];
        res.append(&mut a);
        res
    }

    fn key(&mut self, c: char, ctrl: bool) -> Vec<Response> {
        if ctrl {
            match c {
                'd' => {
                    self.toggle_split();
                    self.refresh()
                },
                _ => vec![],
            }
        } else {
            match c {
                'b' => vec![],
                _ => vec![],
            }
        }
    }
}

impl HSplit {
    fn toggle_split(&mut self) {
        let ws = self.windows.len() % 3 + 1;
        self.set_children(ws);
        let (w, h) = (self.width, self.height);
        self.resize(w, h);
    }

    pub fn set_children(&mut self, children: usize) {
        if children <= self.windows.len() {
            self.windows.truncate(children)
        } else {
            for _ in 0..(children - self.windows.len()) {
                self.windows.push(Child { comp: Box::new(Window::new(0, 0)), x: 0, y: 0 })
            }
        }
    }

    pub fn new(width: usize, height: usize, windows: usize) -> HSplit {
        let mut res = HSplit {
            windows: vec![],
            width: width,
            height: height,
        };
        res.set_children(windows);
        res
    }
}

impl Parent for HSplit {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        self.windows.iter_mut().collect()
    }

    fn children(&self) -> Vec<&Child> {
        self.windows.iter().collect()
    }
}