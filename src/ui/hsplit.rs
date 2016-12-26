use msg::event;
use hq::Hq;
use util::ResultBox;
use ui::res::{Rect, Brush, Color, Response};
use ui::comp::{View, Parent, Component};
use ui::window::Window;

#[derive(Default)]
pub struct HSplit {
    view: View,
    windows: Vec<Window>,
    focused: usize,
}

impl Component for HSplit {
    has_view!();

    /// Resize each child windows.
    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()> {
        let windows = self.windows.len();
        let borders = windows + 1;
        let mut offset = 1;
        for (i, &mut ref mut child) in self.windows.iter_mut().enumerate() {
            let w = (self.view.width - borders + i) / windows;
            child.resize(hq, offset, 1, w, self.view.height - 2)?;
            offset += w + 1;
        }
        Ok(())
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let rect = Rect::new(self.view.width,
                             self.view.height,
                             Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250)));
        self.refresh_children(rect, hq)
    }

    /// Propagate if the event is not handled.
    fn unhandled(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        self.windows[self.focused].propagate(e, hq)
    }

    /// Handle the keyboard event.
    fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        match k {
            event::Key::Ctrl('d') => {
                self.toggle_split(hq)?;
                self.refresh(hq)
            }
            _ => Ok(Response::Unhandled),
        }
    }
}

impl HSplit {
    fn toggle_split(&mut self, hq: &mut Hq) -> ResultBox<()> {
        let ws = self.windows.len() % 3 + 1;
        self.set_children(ws);
        let x = self.view.x;
        let y = self.view.y;
        let w = self.view.width;
        let h = self.view.height;
        self.resize(hq, x, y, w, h)
    }

    pub fn set_children(&mut self, children: usize) {
        // TODO: Must be implemented
        self.focused = 0;
        if children <= self.windows.len() {
            self.windows.truncate(children)
        } else {
            for _ in 0..(children - self.windows.len()) {
                self.windows.push(Window::new_editor())
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
