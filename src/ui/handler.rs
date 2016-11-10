use io::Event;
use ui::screen::Screen;
use ui::comp::Component;
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
            Event::Ctrl { c } => match c {
                'q' => {
                    self.quit = true;
                    vec![Response::Quit]
                },
                _ => self.screen.key(c, true),
            },
            Event::Char { c } => vec![Response::Put(c.to_string())],
            _ => vec![],
        }
    }
}
