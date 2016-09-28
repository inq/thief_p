mod handler;
mod comp;
mod editor;
mod screen;
mod window;
mod res;

use libc;
use util::Chan;
use std::thread;
use std::sync::mpsc;
use io::Event;
use ui::handler::Handler;

pub use ui::res::*;

pub struct Ui {
    chan: Chan<Vec<Response>, Event>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Ui {
    pub fn init() -> Ui {
        unsafe {
            libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
        }

        let (chan, e) = Chan::create();
        let thread = thread::spawn(move || {
            let mut handler = Handler::new();
            while !handler.quit {
                if let Ok(event) = chan.recv() {
                    chan.send(handler.handle(event))
                        .or_else(|e| {
                            println!("{:?}", e);
                            Err(e)
                        })
                        .unwrap()
                }
            }
        });
        Ui {
            chan: e,
            thread: Some(thread),
        }
    }

    pub fn send(&self, e: Event) -> Result<(), mpsc::SendError<Event>> {
        self.chan.send(e)
    }

    pub fn try_recv(&self) -> Result<Vec<Response>, mpsc::TryRecvError> {
        self.chan.try_recv()
    }

    pub fn join(&mut self) -> thread::Result<()> {
        self.thread.take().map(|t| t.join()).unwrap_or(Ok(()))
    }
}
