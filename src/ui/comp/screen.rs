use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{CommandBar, HSplit, Parent, Component, View};

#[derive(Default)]
pub struct Screen {
    view: View,
    hsplit: ScreenChild,
    overlaps: Vec<ScreenChild>,
}

def_child!(ScreenChild <- HSplit, CommandBar);

impl Component for Screen {
    has_view!();

    fn on_resize(&mut self) {
        self.hsplit.resize(1, 1, self.view.width - 2, self.view.height - 2);
        for &mut ref mut child in self.overlaps.iter_mut() {
            child.resize(0, self.view.height - 1, self.view.width, 3);
        }
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
