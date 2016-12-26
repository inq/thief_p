use std::env;
use std::convert::From;
use libc;

use msg::event;
use hq::Hq;
use io::kqueue::Kqueue;
use io::term::Term;
use ui::{Ui, Component, Cursor, Refresh, Sequence};
use util::ResultBox;

def_error! {
    OutOfCapacity: "out of capacity",
    Exit: "exit request",
}

pub struct Handler {
    term: Term,
    ui: Ui,
    hq: Hq,
    ipt_buf: String,
}

impl Handler {
    pub fn new(ui: Ui) -> ResultBox<Handler> {
        Ok(Handler {
            term: Term::new()?,
            hq: Hq::new()?,
            ui: ui,
            ipt_buf: String::with_capacity(32),
        })
    }

    fn handle_stdout(&mut self) -> ResultBox<()> {
        self.term.consume_output_buffer()?;
        Ok(())
    }

    fn handle_stdin(&mut self) -> ResultBox<()> {
        let ipt = self.term.read(32)?;

        if self.ipt_buf.len() + ipt.len() > self.ipt_buf.capacity() {
            return Err(From::from(Error::OutOfCapacity));
        }
        self.ipt_buf.push_str(&ipt);
        let mut cur = self.ipt_buf.clone();
        while let (Some(e), next) = event::Event::from_string(&cur) {
            if let event::Event::Pair(x, y) = e {
                // TODO: check it
                self.term.initial_cursor(&Cursor { x: x, y: y });
                let (w, h) = self.term.get_size()?;
                self.handle_event(event::Event::Resize(w, h))?;
            }
            self.handle_event(e)?;
            cur = next.clone();
        }
        self.ipt_buf.clear();
        self.ipt_buf.push_str(&cur);
        Ok(())
    }

    // Handle event from the Ui.
    fn handle_event(&mut self, e_raw: event::Event) -> ResultBox<()> {
        use ui::Response::*;
        let e = self.hq.preprocess(e_raw);
        let next = match self.ui.propagate(e, &mut self.hq)? {
            Term { refresh, cursor } => {
                self.term.show_cursor(false);
                if let Some(Refresh { x, y, rect }) = refresh {
                    self.term.write_ui_buffer(x, y, &rect);
                }
                if let Some(Cursor { x, y }) = cursor {
                    self.term.move_cursor(x, y);
                }
                self.term.show_cursor(true);
                None
            }
            Command(c) => self.hq.call(&c),
            Quit => {
                self.term.release()?;
                return Err(From::from(Error::Exit));
            }
            Unhandled => None,
        };
        if let Some(e) = next {
            self.handle_event(e)
        } else {
            Ok(())
        }
    }

    // Handle resize event of terminal.
    fn handle_sigwinch(&mut self) -> ResultBox<()> {
        let (w, h) = self.term.get_size()?;
        self.handle_event(event::Event::Resize(w, h))
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
        args.get(1)
            .and_then(|file| {
                self.hq.call("open-file");
                self.hq.call(file)
            })
            .and_then(|e| self.handle_event(e).ok());
        let mut kqueue = Kqueue::new()?;
        kqueue.init()?;
        kqueue.kevent(self)
    }
}
