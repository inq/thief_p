use std::{error, thread};
use std::sync::mpsc::{self, channel};

use io::Event;
use ui::screen::Screen;

struct Handler {
    screen: Screen,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { screen: Screen::new() }
    }

    pub fn handle(&self, e: Event) -> Result<(), Box<error::Error>> {
        match e {
            Event::Char { c: x } => {
                println!("{}", x);
                try!(self.screen.refresh());
            }
            _ => (),
        }
        Ok(())
    }
}

fn do_loop(chan: &mpsc::Receiver<Event>, handler: &Handler) -> Result<(), Box<error::Error>> {
    let res = try!(chan.recv());
    try!(handler.handle(res));
    Ok(())
}

pub fn launch() -> mpsc::Sender<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let handler = Handler::new();
        loop {
            match do_loop(&rx, &handler) {
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    });
    tx
}
