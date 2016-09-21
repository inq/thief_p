use std::thread;
use std::sync::mpsc::{self, channel};

pub fn launch() -> mpsc::Sender<String> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let s = rx.recv().unwrap();
            println!("{}", s);
        };
    });
    tx
}
