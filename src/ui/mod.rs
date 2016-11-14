mod handler;
mod comp;
mod res;

use util::Chan;
use std::{thread, result};
use std::sync::mpsc;
use io::Event;
use ui::handler::Handler;

pub use ui::res::*;

def_error! {
    Initialized: "already initialized",
}

pub struct Ui {
    chan: Chan<Response, Event>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Ui {
    pub fn new() -> Result<Ui> {
        allow_once!();

        let (chan, e) = Chan::create();
        let thread = thread::spawn(move || {
            let mut handler = Handler::new();
            loop {
                if let Ok(event) = chan.recv() {
                    chan.send(handler.handle(event))
                        .or_else(|e| {
                            println!("{:?}", e);
                            Err(e)
                        })
                        .unwrap();
                    if handler.quit {
                        break;
                    }
                }
            }
        });
        Ok(Ui {
            chan: e,
            thread: Some(thread),
        })
    }

    pub fn send(&self, e: Event) -> result::Result<(), mpsc::SendError<Event>> {
        self.chan.send(e)
    }

    pub fn try_recv(&self) -> result::Result<Response, mpsc::TryRecvError> {
        self.chan.try_recv()
    }

    pub fn join(&mut self) -> thread::Result<()> {
        self.thread.take().map(|t| t.join()).unwrap_or(Ok(()))
    }
}

#[test]
fn initialize() {
    assert!(Ui::new().is_ok());
    assert!(Ui::new().is_err());
}
