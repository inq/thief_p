use io::Event;
use ui::res::{Buffer, Brush, Color, Response};
use ui::comp::{CommandBar, HSplit, Parent, Child, Component};

pub struct Screen {
    hsplit: Child,
    overlaps: Vec<Child>,
    width: usize,
    height: usize,
}

impl Component for Screen {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.width = width;
        self.height = height;
        self.hsplit.comp.resize(self.width - 2, self.height - 1);
        self.hsplit.x = 1;
        self.hsplit.y = 0;
        for &mut ref mut child in self.overlaps.iter_mut() {
            child.comp.resize(self.width, 3);
            child.x = 0;
            child.y = self.height - 1;
        }
        (width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        self.refresh_children(buffer)
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Ctrl { c: 'r' } => self.command_bar(),
            _ => {
                let res = self.hsplit.comp.handle(e);
                self.transform(&self.hsplit, res)
            }
        }
    }
}

impl Screen {
    pub fn command_bar(&mut self) -> Response {
        if self.overlaps.len() == 0 {
            let bar = CommandBar::new();
            let res = bar.refresh();
            self.overlaps.push(bar);
            res
        } else {
            Response::empty()
        }
    }

    pub fn new() -> Screen {
        Screen {
            hsplit: HSplit::new(1),
            overlaps: vec![],
            width: usize::max_value(),
            height: usize::max_value(),
        }
    }
}

impl Parent for Screen {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        vec![&mut self.hsplit].into_iter()
            .chain(self.overlaps.iter_mut())
            .collect()
    }

    fn children(&self) -> Vec<&Child> {
        vec![&self.hsplit].into_iter()
            .chain(self.overlaps.iter())
            .collect()
    }
}
