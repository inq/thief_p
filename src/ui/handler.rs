use std::{fmt, error, thread};
use std::sync::mpsc::{self, channel};
use ui::event::Event;

struct Handler {
    buf: String
}

impl Handler {
    pub fn new() -> Result<Handler, Error> {
        Ok(Handler {
            buf: String::with_capacity(32)
        })
    }

    pub fn handle(&self, e: Event) -> Result<(), Error> {
        println!("{:?}", e);
        Ok(())
    }

    pub fn accept(&mut self, ipt: String) -> Result<(), Error> {
        if self.buf.len() + ipt.len() > self.buf.capacity() {
            return Err(Error::OutOfCapacity);
        }
        self.buf.push_str(&ipt);
        let mut cur = self.buf.clone();
        let mut done = false;
        while !done {
            let (res, next) = Event::from_string(cur);
            match res {
                Some(e) => try!(self.handle(e)),
                None => done = true,
            }
            cur = next.clone();
        }
        self.buf.clear();
        self.buf.push_str(&cur);
        Ok(())
    }
}


pub fn launch() -> mpsc::Sender<String> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut handler = Handler::new().unwrap();
        loop {
            let res = rx.recv().unwrap();
            handler.accept(res).unwrap();
        };
    });
    tx
}

#[derive(Debug)]
enum Error {
    OutOfCapacity,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OutOfCapacity => "out of the capacity",
        }
    }
}
