use std::convert::From;
use std::io::{self, Write};
use std::error;
use libc;

use io::{event, input, kqueue};
use io::term::Term;
use ui::{Ui, Brush, Color, Cursor, Response};

def_error! {
    OutOfCapacity: "out of capacity",
}

pub struct Handler {
    term: Term,
    cursor: Option<Cursor>,
    ui: Ui,
    out_buf: String,
}

impl Handler {
    pub fn init(ui: Ui) -> Result<Handler, Box<error::Error>> {
        let term = try!(Term::init());
        try!(::io::signal::init());
        try!(::io::input::init());

        term.smcup();
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
        let ipt = try!(input::read(32));

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
                        if self.cursor.is_none() {
                            self.cursor = Some(Cursor { x: x, y: y })
                        }
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
        // TODO: must use brush as a terminal state
        let br = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));

        if let Ok(resps) = self.ui.try_recv() {
            self.term.clear_output_buffer();
            for resp in resps {
                match resp {
                    Response::Refresh(b) => {
                        self.term.write_ui_buffer(&b);
                    }
                    Response::Move(c) => {
                        self.term.move_cursor(c.x, c.y);
                    }
                    Response::Put(s) => {
                        self.term.write(&s);
                    }
                    Response::Quit => {
                        self.term.rmcup();
                        if let Some(Cursor { x, y }) = self.cursor {
                            self.term.move_cursor(x, y);
                        }
                        self.ui.join().unwrap();
                        try!(io::stdout().flush());
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle(&mut self, ident: usize) -> Result<(), Box<error::Error>> {
        match ident as libc::c_int {
            libc::STDOUT_FILENO => self.handle_stdout(),
            libc::STDIN_FILENO => self.handle_stdin(),
            libc::SIGWINCH => self.handle_sigwinch(),
            kqueue::TIMER_IDENT => self.handle_timer(),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<error::Error>> {
        let mut kqueue = try!(::io::kqueue::Kqueue::new());
        try!(kqueue.init());
        kqueue.kevent(self)
    }
}
