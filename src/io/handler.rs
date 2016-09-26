use std::os::unix::io::RawFd;
use std::convert::From;
use std::sync::mpsc;
use std::{error, str, fmt};
use std::io::{self, Write};
use libc;
use io::{event, input, term};


pub struct Handler {
    pub kq: RawFd,
    pub kq_changes: Vec<libc::kevent>,
    pub kq_events: Vec<libc::kevent>,
    buf: String,
}

const TIMER_IDENT: i32 = 0xbeef;

impl Handler {
    pub fn new() -> Result<Handler, Error> {
        let res = unsafe { libc::kqueue() };
        if res == -1 {
            return Err(Error::Kqueue);
        }
        Ok(Handler {
            kq: res,
            kq_changes: Vec::with_capacity(16),
            kq_events: Vec::with_capacity(16),
            buf: String::with_capacity(32),
        })
    }

    fn add_event(&mut self, ident: i32, filter: i16, aux: isize) {
        self.kq_changes.push(libc::kevent {
            ident: ident as libc::uintptr_t,
            filter: filter,
            flags: libc::EV_ADD,
            fflags: 0,
            data: aux,
            udata: ::std::ptr::null_mut(),
        })
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.add_event(libc::STDIN_FILENO, libc::EVFILT_READ, 0);
        self.add_event(libc::STDOUT_FILENO, libc::EVFILT_WRITE, 0);
        self.add_event(libc::SIGWINCH, libc::EVFILT_SIGNAL, 0);
        self.add_event(TIMER_IDENT, libc::EVFILT_TIMER, 100);
        let res = unsafe {
            libc::kevent(self.kq,
                         self.kq_changes.as_ptr(),
                         self.kq_changes.len() as i32,
                         ::std::ptr::null_mut(),
                         0,
                         &libc::timespec {
                             tv_sec: 10,
                             tv_nsec: 0,
                         })
        };
        if res == -1 {
            return Err(Error::Kevent);
        }
        Ok(())
    }

    pub fn handle(&mut self,
                  chan_output: mpsc::Sender<event::Event>,
                  chan_input: mpsc::Receiver<String>)
                  -> Result<(), Box<error::Error>> {
        let mut buf = Vec::with_capacity(32);
        let mut write_buf = String::new();
        let mut written = 0usize;
        {
            let (w, h) = try!(term::get_size());
            try!(chan_output.send(event::Event::Resize { w: w, h: h }));
        }
        loop {
            let res = unsafe {
                libc::kevent(self.kq,
                             ::std::ptr::null(),
                             0,
                             self.kq_events.as_mut_ptr(),
                             self.kq_events.capacity() as i32,
                             &libc::timespec {
                                 tv_sec: 10,
                                 tv_nsec: 0,
                             })
            };
            if res == -1 {
                return Err(From::from(Error::Kevent));
            }
            unsafe {
                self.kq_events.set_len(res as usize);
            }

            for e in &self.kq_events {
                match e.ident as libc::c_int {
                    libc::STDOUT_FILENO => {
                        if write_buf.len() > written {
                            let (_, v2) = write_buf.split_at(written);
                            let l = try!(io::stdout().write(v2.as_bytes()));
                            written += l;
                            if written == write_buf.len() {
                                try!(io::stdout().flush());
                            }
                        }
                    }
                    libc::STDIN_FILENO => {
                        try!(input::read(&mut buf));
                        let ipt = try!(String::from_utf8(buf.clone()));
                        try!(process(&mut self.buf, &chan_output, ipt));
                    }
                    libc::SIGWINCH => {
                        let (w, h) = try!(term::get_size());
                        try!(chan_output.send(event::Event::Resize{w: w, h: h}));
                    }
                    TIMER_IDENT => {
                        if let Ok(buf) = chan_input.try_recv() {
                            write_buf = buf;
                            written = 0;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

fn process(buf: &mut String,
           chan: &mpsc::Sender<event::Event>,
           ipt: String)
           -> Result<(), Box<error::Error>> {
    if buf.len() + ipt.len() > buf.capacity() {
        return Err(From::from(Error::OutOfCapacity));
    }
    buf.push_str(&ipt);
    let mut cur = buf.clone();
    let mut done = false;
    while !done {
        let (res, next) = event::Event::from_string(cur);
        match res {
            Some(e) => try!(chan.send(e)),
            None => done = true,
        }
        cur = next.clone();
    }
    buf.clear();
    buf.push_str(&cur);
    Ok(())
}


#[derive(Debug)]
pub enum Error {
    Kqueue,
    Kevent,
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
            Error::Kqueue => "kqueue returned -1",
            Error::Kevent => "kevent returned -1",
            Error::OutOfCapacity => "out of the capacity",
        }
    }
}
