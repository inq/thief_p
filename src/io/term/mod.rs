mod output;

use libc;
use std::{error, mem};
use std::io::{self, Write};
use io::term::output::Output;
use ui::{Brush, Buffer};

def_error! {
    Initialized: "already initialized",
    Tcgetattr: "tcgetattr returned -1",
    Tcsetattr: "tcsetattr returned -1",
    Tiocgwinsz: "ioctl returned -1",
}

pub struct Term {
    brush: Option<Brush>,
    output: Output,
}

impl Term {
    pub fn new() -> Result<Term, Error> {
        allow_once!();

        unsafe {
            libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
        }

        let mut termios = unsafe { mem::uninitialized() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut termios) } == -1 {
            return Err(Error::Tcgetattr);
        }
        termios.c_lflag &= !(libc::ICANON);
        termios.c_lflag &= !(libc::ECHO);
        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) } == -1 {
            return Err(Error::Tcsetattr);
        }

        let res = Term {
            brush: None,
            output: Output::new(),
        };
        res.query_cursor();
        Ok(res)
    }

    pub fn clear_output_buffer(&mut self) {
        self.output.clear();
    }

    pub fn consume_output_buffer(&mut self) {
        self.output.consume();
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.output.write(&format!("\u{1b}[{};{}f", y + 1, x + 1));
    }

    pub fn write(&mut self, s: &String) {
        self.output.write(s)
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

    pub fn smcup(&self) {
        print!("\u{1b}[?47h");
    }

    pub fn rmcup(&self) {
        print!("\u{1b}[?47l");
    }

    pub fn query_cursor(&self) {
        print!("\u{1b}[6n");
    }
}

#[test]
fn initialize() {
    assert!(Term::new().is_ok());
    assert!(Term::new().is_err());
}
