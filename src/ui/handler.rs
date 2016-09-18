use std::thread;
use std::sync::mpsc::{self, channel};
use ui::event::Event;

pub fn launch() -> mpsc::Sender<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let e = rx.recv().unwrap();
            println!("{:?}", e);
        }
    });
    tx
}
