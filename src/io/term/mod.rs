mod output;

use libc;
use std::{error, mem};
use std::io::{self, Write};
use io::term::output::Output;
use ui::{Brush, Buffer, Cursor};

def_error! {
    Initialized: "already initialized",
    Tcgetattr: "tcgetattr returned -1",
    Tcsetattr: "tcsetattr returned -1",
    Tiocgwinsz: "ioctl returned -1",
    Read: "read returned -1",
    FGetfl: "fcntl(F_GETFL) returned -1",
    FSetfl: "fcntl(F_SETFL) returned -1",
}

pub struct Term {
    buffering: bool,
    initial_cursor: Option<Cursor>,
    brush: Option<Brush>,
    output: Output,
}

impl Term {
    pub fn new() -> Result<Term, Error> {
        allow_once!();
        let mut term = Term {
            buffering: true,
            initial_cursor: None,
            brush: None,
            output: Output::new(),
        };
        term.echo(false);
        term.query_cursor();
        term.buffering(false);
        io::stdout().flush();
        Ok(term)
    }

    pub fn release(&mut self) {
        self.buffering(true);
        self.echo(true);
        if let Some(ref cursor) = self.initial_cursor.clone() {
            self.move_cursor(cursor.x + 1, cursor.y + 1);
        }
        self.rmcup();
        io::stdout().flush();
    }

    pub fn initial_cursor(&mut self, cursor: &Cursor) {
        if self.initial_cursor.is_none() {
            self.initial_cursor = Some((*cursor).clone());
            self.smcup();
        }
    }

    pub fn clear_output_buffer(&mut self) {
        self.output.clear();
    }

    pub fn consume_output_buffer(&mut self) {
        self.output.consume();
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.write(&format!("\u{1b}[{};{}f", y + 1, x + 1));
    }

    pub fn write(&mut self, s: &String) {
        if self.buffering {
            io::stdout().write(s.as_bytes());
        } else {
            self.output.write(s);
        }
    }

    pub fn write_ui_buffer(&mut self, s: &Buffer) {
        self.output.write(&s.to_string(&mut self.brush))
    }

    pub fn get_size(&self) -> Result<(usize, usize), Error> {
        let ws: libc::winsize = unsafe { mem::uninitialized() };
        let res = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws) };
        if res < 0 {
            return Err(Error::Tiocgwinsz);
        }
        Ok((ws.ws_col as usize, ws.ws_row as usize))
    }

    pub fn smcup(&mut self) {
        self.write(&String::from("\u{1b}[?47h"));
    }

    pub fn rmcup(&mut self) {
        self.write(&String::from("\u{1b}[?47l"));
    }

    pub fn query_cursor(&mut self) {
        self.write(&String::from("\u{1b}[6n"));
    }

    pub fn read(&self, limit: usize) -> Result<String, Box<error::Error>> {
        let mut buf = Vec::with_capacity(limit);
        unsafe {
            let res = libc::read(libc::STDIN_FILENO,
                                 buf.as_mut_ptr() as *mut libc::c_void,
                                 buf.capacity() as usize);
            if res < 0 {
                return Err(From::from(Error::Read));
            }
            buf.set_len(res as usize);
        }
        Ok(try!(String::from_utf8(buf)))
    }

    fn buffering(&mut self, on: bool) -> Result<(), Error> {
        unsafe {
            let prev = libc::fcntl(libc::STDIN_FILENO, libc::F_GETFL);
            if prev == -1 {
                return Err(Error::FGetfl);
            }
            let res = if on {
                libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, prev | libc::O_NONBLOCK)
            } else {
                libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, prev | libc::O_NONBLOCK)
            };
            if res == -1 {
                return Err(Error::FSetfl);
            }
        }
        self.buffering = on;
        Ok(())
    }

    fn echo(&mut self, on: bool) -> Result<(), Error> {
        unsafe {
            libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
            let mut termios = mem::uninitialized();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut termios) == -1 {
                return Err(Error::Tcgetattr);
            }
            if on {
                termios.c_lflag |= libc::ICANON;
                termios.c_lflag |= libc::ECHO;
            } else {
                termios.c_lflag &= !libc::ICANON;
                termios.c_lflag &= !libc::ECHO;
            }
            if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) == -1 {
                return Err(Error::Tcsetattr);
            }
        }
        Ok(())
    }
}

#[test]
fn initialize() {
    assert!(Term::new().is_ok());
    assert!(Term::new().is_err());
}
