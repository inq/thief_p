use io::Event;
use ui::comp::{Component, Screen};
use ui::res::Response;

pub struct Handler {
    screen: Screen,
    pub quit: bool,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            screen: Screen::new(),
            quit: false,
        }
    }

    /// Handle the event, and return the series of responses.
    pub fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(0, 0, width, height);
                self.screen.refresh()
            },
            Event::Ctrl { c: 'q' } => {
                self.quit = true;
                Response::quit()
            },
            _ => self.screen.handle(e),
        }
    }
}
