use io::Event;
use ui::comp::{Component, Screen};
use ui::res::Response;

pub struct Handler {
    screen: Screen,
    pub quit: bool,
}

impl Handler {
    pub fn new() -> Handler {
        let scr = Screen::new();
        Handler {
            screen: scr,
            quit: false,
        }
    }

    /// Handle the event, and return the series of responses.
    pub fn handle(&mut self, e: Event) -> Vec<Response> {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(width, height);
                self.screen.refresh()
            },
            Event::Ctrl { c: 'q' } => {
                self.quit = true;
                vec![Response::Quit]
            },
            _ => self.screen.handle(e),
        }
    }
}
