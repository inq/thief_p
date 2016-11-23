use io::Event;
use hq::Hq;
use ui::comp::{Component, Screen};
use ui::res::Response;

pub struct Handler {
    screen: Screen,
    hq: Hq,
    pub quit: bool,
}

impl Handler {
    pub fn new() -> Handler {
        let mut hq: Hq = Default::default();
        let _ = hq.open_file("LICENSE");
        Handler {
            screen: Screen::new(),
            hq: hq,
            quit: false,
        }
    }

    /// Handle the event, and return the series of responses.
    pub fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(0, 0, width, height);
                self.screen.refresh(&mut self.hq)
            },
            Event::Ctrl { c: 'q' } => {
                self.quit = true;
                Response::quit()
            },
            _ => self.screen.handle(e, &mut self.hq),
        }
    }
}
