use std::{thread};
use std::sync::mpsc::{self, channel};
use io::Event;

struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn handle(&self, e: Event) {
        println!("{:?}", e);
    }
}


pub fn launch() -> mpsc::Sender<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let handler = Handler::new();
        loop {
            let res = rx.recv().unwrap();
            handler.handle(res);
        };
    });
    tx
}
