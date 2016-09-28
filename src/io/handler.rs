use std::convert::From;
use std::io::{self, Write};
use std::error;
use libc;
use io::{event, input, term, kqueue};
use ui::{Ui, Brush, Color, Cursor, Response};

def_error! {
    OutOfCapacity: "out of capacity",
}

pub struct Handler {
    cursor: Option<Cursor>,
    ui: Ui,
    ipt_buf: String,
    ipt_written: usize,
    out_buf: String,
}

impl Handler {
    pub fn init(ui: Ui) -> Result<Handler, Box<error::Error>> {
        try!(::io::signal::init());
        try!(::io::input::init());
        try!(::io::term::init());

        term::smcup();
        let (w, h) = try!(term::get_size());
        try!(ui.send(event::Event::Resize { w: w, h: h }));

        Ok(Handler {
            cursor: None,
            ui: ui,
            ipt_buf: String::with_capacity(4096),
            ipt_written: 0,
            out_buf: String::with_capacity(32),
        })
    }

    fn handle_stdout(&mut self) -> Result<(), Box<error::Error>> {
        if self.ipt_buf.len() > self.ipt_written {
            let (_, v2) = self.ipt_buf.split_at(self.ipt_written);
            let l = try!(io::stdout().write(v2.as_bytes()));
            self.ipt_written += l;
            if self.ipt_written == self.ipt_buf.len() {
                try!(io::stdout().flush());
            }
        }
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
        let (w, h) = try!(term::get_size());
        try!(self.ui.send(event::Event::Resize { w: w, h: h }));
        Ok(())
    }

    fn handle_timer(&mut self) -> Result<(), Box<error::Error>> {
        // TODO: must use brush as a terminal state
        let br = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));

        if let Ok(resps) = self.ui.try_recv() {
            self.ipt_buf.clear();
            for resp in resps {
                match resp {
                    Response::Refresh(b) => {
                        b.print(&mut self.ipt_buf, &br.invert());
                    }
                    Response::Move(c) => {
                        self.ipt_buf.push_str(&term::movexy(c.x, c.y));
                    }
                    Response::Put(s) => {
                        self.ipt_buf.push_str(&s);
                    }
                    Response::Quit => {
                        term::rmcup();
                        if let Some(Cursor { x, y }) = self.cursor {
                            print!("{}", term::movexy(x, y));
                        }
                        self.ui.join().unwrap();
                        try!(io::stdout().flush());
                        return Ok(());
                    }
                }
            }
            self.ipt_written = 0;
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
