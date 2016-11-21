use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{CommandBar, HSplit, Parent, Component, View};

#[derive(Default)]
pub struct Screen {
    view: View,
    hsplit: ScreenChild,
    command_bar: ScreenChild,
    show_command_bar: bool,
}

def_child!(ScreenChild <- HSplit, CommandBar);

impl Component for Screen {
    has_view!();

    fn on_resize(&mut self) {
        self.hsplit.resize(1, 1, self.view.width - 2, self.view.height - 2);
        self.resize_command_bar();
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(80, 0, 0));
        let buffer = Buffer::blank(&b, self.view.width, self.view.height);
        self.refresh_children(buffer)
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Ctrl { c: 'c' } => self.activate_command_bar(),
            _ => self.hsplit.propagate(e)
        }
    }
}

impl Screen {
    /// Resize the command bar; the bottom-side of the screen.
    #[inline]
    fn resize_command_bar(&mut self) {
        self.command_bar.resize(0, self.view.height - 1, self.view.width, 1);
    }

    /// Activate command bar, and redrew the corresponding area.
    #[inline]
    pub fn activate_command_bar(&mut self) -> Response {
        self.show_command_bar = true;
        self.resize_command_bar();
        // TODO: Make concise.
        self.command_bar.refresh().translate(
            self.command_bar.get_view().x,
            self.command_bar.get_view().y)
    }

    pub fn new() -> Screen {
        Screen {
            hsplit: ScreenChild::HSplit(HSplit::new(1)),
            command_bar: ScreenChild::CommandBar(Default::default()),
            ..Default::default()
        }
    }
}

impl Parent for Screen {
    type Child = ScreenChild;
    fn children_mut(&mut self) -> Vec<&mut ScreenChild> {
        if self.show_command_bar {
            vec![&mut self.hsplit, &mut self.command_bar].into_iter()
                .collect()
        } else {
            vec![&mut self.hsplit].into_iter()
                .collect()
        }
    }

    fn children(&self) -> Vec<&ScreenChild> {
        if self.show_command_bar {
            vec![&self.hsplit, &self.command_bar].into_iter()
                .collect()
        } else {
            vec![&self.hsplit].into_iter()
                .collect()
        }
    }
}
