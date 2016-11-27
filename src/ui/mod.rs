mod comp;
mod res;

use util::Chan;
use std::{thread, result};
use std::sync::mpsc;
use io::Event;
use hq::Hq;
use ui::comp::{Component, Screen};

pub use ui::res::*;

def_error! {
    Initialized: "already initialized",
}

pub struct Ui {
    screen: Screen,
    hq: Hq,
}

impl Ui {
    /// Handle the event, and return the series of responses.
    pub fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(0, 0, width, height);
                self.screen.refresh(&mut self.hq)
            },
            Event::Ctrl { c: 'q' } => {
                Response::quit()
            },
            _ => self.screen.handle(e, &mut self.hq),
        }
    }

    pub fn new() -> Result<Ui> {
        allow_once!();
        let mut hq = Hq::new();
        let _ = hq.cmd("open-file", "LICENSE")
            .unwrap_or_else(|x| panic!(String::from(x.description())));
        Ok(Ui {
            screen: Screen::new(),
            hq: hq,
        })
    }
}

#[test]
fn initialize() {
    assert!(Ui::new().is_ok());
    assert!(Ui::new().is_err());
}
