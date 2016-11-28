use hq::Hq;
use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{View, Parent, Component, Window};

#[derive(Default)]
pub struct HSplit {
    view: View,
    windows: Vec<Window>,
    focused: usize,
}

impl Component for HSplit {
    has_view!();

    /// Resize each child windows.
    fn on_resize(&mut self) {
        let windows = self.windows.len();
        let borders = windows + 1;
        let mut offset = 1;
        for (i, &mut ref mut child) in self.windows.iter_mut().enumerate() {
            let w = (self.view.width - borders + i) / windows;
            child.resize(offset, 1, w, self.view.height - 2);
            offset += w + 1;
        }
    }

    fn refresh(&self, hq: &mut Hq) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let buffer = Buffer::blank(&b, self.view.width, self.view.height);
        self.refresh_children(buffer, hq)
    }

    fn handle(&mut self, e: Event, hq: &mut Hq) -> Response {
        match e {
            Event::Ctrl { c: 'd' } => {
                self.toggle_split();
                self.refresh(hq)
            }
            _ => self.windows[self.focused].propagate(e, hq),
        }
    }
}

impl HSplit {
    fn toggle_split(&mut self) {
        let ws = self.windows.len() % 3 + 1;
        self.set_children(ws);
        let x = self.view.x;
        let y = self.view.y;
        let w = self.view.width;
        let h = self.view.height;
        self.resize(x, y, w, h);
    }

    pub fn set_children(&mut self, children: usize) {
        // TODO: Must be implemented
        self.focused = 0;
        if children <= self.windows.len() {
            self.windows.truncate(children)
        } else {
            for _ in 0..(children - self.windows.len()) {
                self.windows.push(Window::new_edit())
            }
        }
    }

    pub fn new(windows: usize) -> HSplit {
        let mut res: HSplit = Default::default();
        res.set_children(windows);
        res
    }
}

impl Parent for HSplit {
    type Child = Window;
    fn children_mut(&mut self) -> Vec<&mut Window> {
        self.windows.iter_mut().collect()
    }

    fn children(&self) -> Vec<&Window> {
        self.windows.iter().collect()
    }
}
