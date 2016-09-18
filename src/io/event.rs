use std::os::unix::io::{RawFd};
use std::error::{self, Error};
use std::convert::From;
use std::fmt;
use libc;

pub struct Event {
    pub kq: RawFd,
    pub changes: Vec<libc::kevent>,
    pub events: Vec<libc::kevent>,
}

#[derive(Debug)]
pub enum EventError {
    KqueueError,
    KeventError,
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl error::Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::KqueueError => "kqueue returned -1",
            EventError::KeventError => "kevent returned -1",
        }
    }
}

impl Event {
    pub fn new() -> Result<Event, EventError> {
        let res = unsafe { libc::kqueue() };
        if res == -1 {
            return Err(EventError::KqueueError);
        }
        Ok(Event {
            kq: res,
            changes: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
        })
    }

    fn add_fd(&mut self, fd: RawFd) {
        self.changes.push(libc::kevent {
            ident: fd as libc::uintptr_t,
            filter: libc::EVFILT_READ,
            flags: libc::EV_ADD,
            fflags: 0,
            data: 0,
            udata: ::std::ptr::null_mut()
        })
    }

    pub fn init(&mut self) -> Result<(), EventError> {
        self.add_fd(libc::STDIN_FILENO);
        let res = unsafe {
            libc::kevent(
                self.kq,
                self.changes.as_ptr(),
                self.changes.len() as i32,
                ::std::ptr::null_mut(),
                0,
                &libc::timespec { tv_sec: 10, tv_nsec: 0})
        };
        if res == -1 {
            return Err(EventError::KeventError);
        }
        Ok(())
    }

    pub fn handle<T>(&mut self, handler: T) -> Result<(), Box<Error>>
        where T : Fn(&libc::kevent) -> Result<(), Box<Error>> {
        let res = unsafe {
            libc::kevent(
                self.kq,
                ::std::ptr::null(),
                0,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                &libc::timespec { tv_sec: 10, tv_nsec: 0 })
        };
        if res == -1 {
            return Err(From::from(EventError::KeventError));
        }
        unsafe {
            self.events.set_len(res as usize);
        }
        for ev in &self.events {
            try!(handler(ev));
        }
        Ok(())
    }
}
