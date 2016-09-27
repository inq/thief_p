use std::error::Error;

mod event;
mod handler;
mod input;
mod signal;
mod term;
mod kqueue;

pub use io::event::Event;
use std::sync::mpsc;
use ui::{Brush, Color, Response};

pub fn init() -> Result<(usize, usize), Box<Error>> {
    try!(signal::init());
    try!(input::init());
    let res = try!(term::init());
    Ok(res)
}

pub fn run(out: mpsc::Sender<event::Event>,
           ipt: mpsc::Receiver<Vec<Response>>)
           -> Result<(), Box<Error>> {
    let mut kqueue = try!(::io::kqueue::Kqueue::new());
    let mut handler = ::io::handler::Handler::new(out, ipt);
    try!(kqueue.init());
    try!(handler.init());
    kqueue.kevent(&mut handler)
}
