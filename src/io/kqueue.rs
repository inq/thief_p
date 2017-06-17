use std::os::unix::io::RawFd;
use libc;
use io::handler::Handler;
use util::ResultBox;

def_error! {
    Kqueue: "kqueue returned -1",
    Kevent: "kevent returned -1",
}

pub struct Kqueue {
    kq: RawFd,
    changes: Vec<libc::kevent>,
    events: Vec<libc::kevent>,
}

pub const TIMER_IDENT: i32 = 0xbeef;

impl Kqueue {
    /// Initialize the kqueue file descripter, changeset, and eventset.
    pub fn new() -> Result<Kqueue> {
        let res = unsafe { libc::kqueue() };
        if res == -1 {
            return Err(Error::Kqueue);
        }
        Ok(Kqueue {
            kq: res,
            changes: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
        })
    }

    /// Fetch kevent into the eventset.
    fn fetch_events(&mut self) -> Result<()> {
        unsafe {
            let res = libc::kevent(
                self.kq,
                ::std::ptr::null(),
                0,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                &libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
            );
            if res == -1 {
                return Err(Error::Kevent);
            } else {
                self.events.set_len(res as usize);
            }
        }
        Ok(())
    }

    /// Add a new kevent into the changeset.
    #[inline]
    fn add_event(&mut self, ident: i32, filter: i16, aux: isize) {
        self.changes.push(libc::kevent {
            ident: ident as libc::uintptr_t,
            filter: filter,
            flags: libc::EV_ADD,
            fflags: 0,
            data: aux,
            udata: ::std::ptr::null_mut(),
        })
    }

    /// Initialize the kqueue API with the changeset.
    /// changeset: [STDIN, STDOUT, TIMER, SIGWINCH]
    pub fn init(&mut self) -> Result<()> {
        self.add_event(libc::STDIN_FILENO, libc::EVFILT_READ, 0);
        self.add_event(libc::STDOUT_FILENO, libc::EVFILT_WRITE, 0);
        self.add_event(libc::SIGWINCH, libc::EVFILT_SIGNAL, 0);
        self.add_event(TIMER_IDENT, libc::EVFILT_TIMER, 100);
        let res = unsafe {
            libc::kevent(
                self.kq,
                self.changes.as_ptr(),
                self.changes.len() as i32,
                ::std::ptr::null_mut(),
                0,
                &libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
            )
        };
        if res == -1 {
            Err(Error::Kevent)
        } else {
            Ok(())
        }
    }

    /// Handle the kevent.
    pub fn kevent(&mut self, handler: &mut Handler) -> ResultBox<()> {
        loop {
            self.fetch_events()?;
            for e in &self.events {
                handler.handle(e.ident as usize)?;
            }
        }
    }
}
