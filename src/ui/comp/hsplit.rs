use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{EditWindow, Parent, Child, Component};

pub struct HSplit {
    windows: Vec<Child>,
    focused: usize,
    width: usize,
    height: usize,
}

impl Component for HSplit {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.width = width;
        self.height = height;
        let borders = self.windows.len() + 1;
        let windows = self.windows.len();
        let mut offset = 0;
        for (i, &mut ref mut child) in self.windows.iter_mut().enumerate() {
            let w = (self.width - borders + i + 1) / windows;
            child.comp.resize(w, self.height - 2);
            child.x = offset;
            child.y = 1;
            offset += w + 1;
        }
        (width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let buffer = Buffer::blank(&b, self.width, self.height);
        self.refresh_children(buffer)
    }

    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Ctrl { c: 'd' } => {
                self.toggle_split();
                self.refresh()
            },
            _ => {
                let res = self.windows[self.focused].comp.handle(e);
                self.transform(&self.windows[self.focused], res)
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
        // TODO: Must be implemented
        self.focused = 0;
        if children <= self.windows.len() {
            self.windows.truncate(children)
        } else {
            for _ in 0..(children - self.windows.len()) {
                self.windows.push(EditWindow::new())
            }
        }
    }

    pub fn new(windows: usize) -> Child {
        let mut res = HSplit {
            windows: vec![],
            focused: usize::max_value(),
            width: usize::max_value(),
            height: usize::max_value(),
        };
        res.set_children(windows);
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(res),
        }
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
