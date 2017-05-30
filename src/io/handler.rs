use std::env;
use std::convert::From;
use libc;

use term;
use hq;
use io::kqueue::Kqueue;
use io::term::Term;
use util::ResultBox;

def_error! {
    OutOfCapacity: "out of capacity",
    Exit: "exit request",
}

pub struct Handler {
    term: Term,
    hq: hq::Handler,
    ipt_buf: String,
}

impl Handler {
    pub fn new(hq_handler: hq::Handler) -> ResultBox<Handler> {
        Ok(Handler {
               hq: hq_handler,
               term: Term::new()?,
               ipt_buf: String::with_capacity(32),
           })
    }

    /// STDOUT - Consume the output buffer.
    fn handle_stdout(&mut self) -> ResultBox<()> {
        self.term.consume_output_buffer()?;
        Ok(())
    }

    /// STDIN - Fetch from the read buffer.
    fn handle_stdin(&mut self) -> ResultBox<()> {
        let ipt = self.term.read(32)?;

        if self.ipt_buf.len() + ipt.len() > self.ipt_buf.capacity() {
            return Err(From::from(Error::OutOfCapacity));
        }
        self.ipt_buf.push_str(&ipt);
        let mut cur = self.ipt_buf.clone();
        while let (Some(e), next) = hq::Request::from_string(&cur) {
            if let hq::Request::Pair(x, y) = e {
                // TODO: check this
                self.term.initial_cursor(&(x, y));
                let (w, h) = self.term.get_size()?;
                self.handle_event(hq::Request::Resize(w, h))?;
            }
            self.handle_event(e)?;
            cur = next.clone();
        }
        self.ipt_buf.clear();
        self.ipt_buf.push_str(&cur);
        Ok(())
    }

    /// Handle result from the Hq.
    fn handle_event(&mut self, e: hq::Request) -> ResultBox<()> {
        match self.hq.request(e)? {
            //hq::Response::Command(c) => self.hq.call(&c),
            hq::Response::Term { refresh, cursor } => {
                self.term.show_cursor(false);
                if let Some(term::Refresh { x, y, rect }) = refresh {
                    self.term.write_ui_buffer(x, y, &rect);
                }
                if let Some((x, y)) = cursor {
                    self.term.move_cursor(x, y);
                }
                self.term.show_cursor(true);
            }
            hq::Response::Quit => {
                self.term.release()?;
                return Err(From::from(Error::Exit));
            }
            _ => (),
        };
        Ok(())
    }

    // Handle resize event of terminal.
    fn handle_sigwinch(&mut self) -> ResultBox<()> {
        let (w, h) = self.term.get_size()?;
        self.handle_event(hq::Request::Resize(w, h))
    }

    pub fn handle(&mut self, ident: usize) -> ResultBox<()> {
        match ident as libc::c_int {
            libc::STDOUT_FILENO => self.handle_stdout(),
            libc::STDIN_FILENO => self.handle_stdin(),
            libc::SIGWINCH => self.handle_sigwinch(),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self) -> ResultBox<()> {
        let args: Vec<String> = env::args().collect();
        /*
        args.get(1)
            .and_then(|file| {
                          self.hq.call("open-file");
                          self.hq.call(file)
                      })
            .and_then(|e| self.handle_event(e).ok());
         */
        let mut kqueue = Kqueue::new()?;
        kqueue.init()?;
        kqueue.kevent(self)
    }
}
