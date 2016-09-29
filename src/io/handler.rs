use std::convert::From;
use std::io::{self, Write};
use std::error;
use libc;

use io::{event, kqueue};
use io::term::Term;
use ui::{Ui, Brush, Color, Cursor, Response};

def_error! {
    OutOfCapacity: "out of capacity",
    Exit: "exit request",
}

pub struct Handler {
    term: Term,
    cursor: Option<Cursor>,
    ui: Ui,
    out_buf: String,
}

impl Handler {
    pub fn new(ui: Ui) -> Result<Handler, Box<error::Error>> {
        let term = try!(Term::new());
        try!(::io::signal::init());

        let (w, h) = try!(term.get_size());
        try!(ui.send(event::Event::Resize { w: w, h: h }));

        Ok(Handler {
            term: term,
            cursor: None,
            ui: ui,
            out_buf: String::with_capacity(32),
        })
    }

    fn handle_stdout(&mut self) -> Result<(), Box<error::Error>> {
        self.term.consume_output_buffer();
        Ok(())
    }

    fn handle_stdin(&mut self) -> Result<(), Box<error::Error>> {
        let ipt = try!(self.term.read(32));

        if self.out_buf.len() + ipt.len() > self.out_buf.capacity() {
            return Err(From::from(Error::OutOfCapacity));
        }
        self.out_buf.push_str(&ipt);
        let mut cur = self.out_buf.clone();
        let mut done = false;
        while !done {
            let (res, next) = event::Event::from_string(cur);
            match res {
                Some(e) => {
                    if let event::Event::Pair { x, y } = e {
                        self.term.initial_cursor(&Cursor { x: x, y: y });
                    }
                    try!(self.ui.send(e))
                }
                None => done = true,
            }
            cur = next.clone();
        }
        self.out_buf.clear();
        self.out_buf.push_str(&cur);
        Ok(())
    }

    fn handle_sigwinch(&mut self) -> Result<(), Box<error::Error>> {
        let (w, h) = try!(self.term.get_size());
        try!(self.ui.send(event::Event::Resize { w: w, h: h }));
        Ok(())
    }

    fn handle_timer(&mut self) -> Result<(), Box<error::Error>> {
        if let Ok(resps) = self.ui.try_recv() {
            for resp in resps {
                match resp {
                    Response::Refresh(b) => {
                        self.term.move_cursor(0, 0);
                        self.term.write_ui_buffer(&b);
                    }
                    Response::Move(c) => {
                        self.term.move_cursor(c.x, c.y);
                    }
                    Response::Put(s) => {
                        self.term.write(&s);
                    }
                    Response::Quit => {
                        self.ui.join().unwrap();
                        self.term.release();
                        return Err(From::from(Error::Exit));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle(&mut self, ident: usize) -> Result<(), Box<error::Error>> {
        try!(self.handle_timer());
        match ident as libc::c_int {
            libc::STDOUT_FILENO => self.handle_stdout(),
            libc::STDIN_FILENO => self.handle_stdin(),
            libc::SIGWINCH => self.handle_sigwinch(),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<error::Error>> {
        let mut kqueue = try!(::io::kqueue::Kqueue::new());
        try!(kqueue.init());
        kqueue.kevent(self)
    }
}
