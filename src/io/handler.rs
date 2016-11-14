use std::convert::From;
use libc;

use io::kqueue::Kqueue;
use io::event::Event;
use io::term::Term;
use ui::{Ui, Cursor, Response, Refresh, Sequence};
use util::ResultBox;

def_error! {
    OutOfCapacity: "out of capacity",
    Exit: "exit request",
}

pub struct Handler {
    term: Term,
    ui: Ui,
    ipt_buf: String,
}

impl Handler {
    pub fn new(ui: Ui) -> ResultBox<Handler> {
        let term = try!(Term::new());
        try!(::io::signal::init());

        let (w, h) = try!(term.get_size());
        try!(ui.send(Event::Resize { w: w, h: h }));

        Ok(Handler {
            term: term,
            ui: ui,
            ipt_buf: String::with_capacity(32),
        })
    }

    fn handle_stdout(&mut self) -> ResultBox<()> {
        try!(self.term.consume_output_buffer());
        Ok(())
    }

    fn handle_stdin(&mut self) -> ResultBox<()> {
        let ipt = try!(self.term.read(32));

        if self.ipt_buf.len() + ipt.len() > self.ipt_buf.capacity() {
            return Err(From::from(Error::OutOfCapacity));
        }
        self.ipt_buf.push_str(&ipt);
        let mut cur = self.ipt_buf.clone();
        let mut done = false;
        while !done {
            let (res, next) = Event::from_string(cur);
            match res {
                Some(e) => {
                    if let Event::Pair { x, y } = e {
                        self.term.initial_cursor(&Cursor { x: x, y: y });
                    }
                    try!(self.ui.send(e))
                }
                None => done = true,
            }
            cur = next.clone();
        }
        self.ipt_buf.clear();
        self.ipt_buf.push_str(&cur);
        Ok(())
    }

    fn handle_sigwinch(&mut self) -> ResultBox<()> {
        let (w, h) = try!(self.term.get_size());
        try!(self.ui.send(Event::Resize { w: w, h: h }));
        Ok(())
    }

    fn handle_timer(&mut self) -> ResultBox<()> {
        if let Ok(resp) = self.ui.try_recv() {
            if let Some(Refresh { x, y, buf }) = resp.refresh {
                self.term.write_ui_buffer(x, y, &buf);
            }
            for resp in resp.sequence {
                match resp {
                    Sequence::Move(c) => {
                        self.term.move_cursor(c.x, c.y);
                    }
                    Sequence::Put(s) => {
                        self.term.write(&s);
                    }
                    Sequence::Line(l) => {
                        self.term.write_ui_line(&l);
                    }
                    Sequence::Show(b) => {
                        self.term.show_cursor(b);
                    }
                    Sequence::Quit => {
                        self.ui.join().unwrap();
                        try!(self.term.release());
                        return Err(From::from(Error::Exit));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle(&mut self, ident: usize) -> ResultBox<()> {
        try!(self.handle_timer());
        match ident as libc::c_int {
            libc::STDOUT_FILENO => self.handle_stdout(),
            libc::STDIN_FILENO => self.handle_stdin(),
            libc::SIGWINCH => self.handle_sigwinch(),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self) -> ResultBox<()> {
        let mut kqueue = try!(Kqueue::new());
        try!(kqueue.init());
        kqueue.kevent(self)
    }
}
