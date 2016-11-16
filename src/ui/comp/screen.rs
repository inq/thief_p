use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{CommandBar, HSplit, Parent, Component, View};

#[derive(Default)]
pub struct Screen {
    view: View,
    hsplit: ScreenChild,
    overlaps: Vec<ScreenChild>,
}

enum ScreenChild {
    CommandBar(CommandBar),
    HSplit(HSplit),
}

impl Default for ScreenChild {
    fn default() -> ScreenChild {
        ScreenChild::HSplit(Default::default())
    }
}

impl Component for Screen {
    fn get_view(&self) -> &View {
        &self.view
    }

    fn resize(&mut self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        self.view.x = x;
        self.view.y = y;
        self.view.width = width;
        self.view.height = height;
        self.hsplit.resize(1, 1, width - 2, height - 2);
        for &mut ref mut child in self.overlaps.iter_mut() {
            child.resize(0, height - 1, width, 3);
        }
        (width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(80, 0, 0));
        let buffer = Buffer::blank(&b, self.view.width, self.view.height);
        self.refresh_children(buffer)
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Ctrl { c: 'r' } => self.command_bar(),
            _ => {
                let res = self.hsplit.handle(e);
                self.hsplit.transform(res)
            }
        }
    }
}

impl Screen {
    pub fn command_bar(&mut self) -> Response {
        if self.overlaps.len() == 0 {
            let bar = ScreenChild::CommandBar(Default::default());
            let res = bar.refresh();
            self.overlaps.push(bar);
            res
        } else {
            Default::default()
        }
    }

    pub fn new() -> Screen {
        Screen {
            hsplit: ScreenChild::HSplit(HSplit::new(1)),
            ..Default::default()
        }
    }
}

impl Parent for Screen {
    type Child = ScreenChild;
    fn children_mut<'a>(&'a mut self) -> Vec<&'a mut ScreenChild> {
        vec![&mut self.hsplit].into_iter()
            .chain(self.overlaps.iter_mut())
            .collect()
    }

    fn children(&self) -> Vec<&ScreenChild> {
        vec![&self.hsplit].into_iter()
            .chain(self.overlaps.iter())
            .collect()
    }
}


impl Component for ScreenChild {
    fn get_view(&self) -> &View {
        match *self {
            ScreenChild::CommandBar(ref sc) => sc.get_view(),
            ScreenChild::HSplit(ref sc) => sc.get_view(),
        }
    }

    fn resize(&mut self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        match *self {
            ScreenChild::CommandBar(ref mut sc) => sc.resize(x, y, width, height),
            ScreenChild::HSplit(ref mut sc) => sc.resize(x, y, width, height),
        }
    }

    fn refresh(&self) -> Response {
        match *self {
            ScreenChild::CommandBar(ref sc) => sc.refresh(),
            ScreenChild::HSplit(ref sc) => sc.refresh(),
        }
    }

    fn handle(&mut self, e: Event) -> Response {
        match *self {
            ScreenChild::CommandBar(ref mut sc) => sc.handle(e),
            ScreenChild::HSplit(ref mut sc) => sc.handle(e),
        }
    }
}
