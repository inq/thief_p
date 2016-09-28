use std::{error, thread};
use std::sync::mpsc;

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
        Handler {
            screen: Screen::new(10, 10),
            quit: false,
        }
    }

    pub fn handle(&mut self, e: Event) -> Vec<Response> {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(width, height);
                self.screen.refresh()
            }
            Event::Ctrl { c: c } => {
                match c {
                    'q' => {
                        self.quit = true;
                        vec![Response::Quit]
                    }
                    _ => vec![],
                }
            }
            Event::Char { c: c } => vec![Response::Put(format!("{}", c))],
            _ => vec![],
        }
    }
}
