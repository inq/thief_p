mod output;

use libc;
use std::mem;
use std::io::{self, Write};
use io::term::output::Output;
use ui::{Brush, Line, Char, Rect, Cursor};
use util::ResultBox;

def_error! {
    Initialized: "already initialized",
    Tcgetattr: "tcgetattr returned -1",
    Tcsetattr: "tcsetattr returned -1",
    Tiocgwinsz: "ioctl returned -1",
    Read: "read returned -1",
    FGetfl: "fcntl(F_GETFL) returned -1",
    FSetfl: "fcntl(F_SETFL) returned -1",
}

#[derive(Default)]
pub struct Term {
    buffering: bool,
    initial_cursor: Option<Cursor>,
    brush: Option<Brush>,
    output: Output,
}

impl Term {
    pub fn new() -> ResultBox<Term> {
        allow_once!();
        let mut term: Term = Default::default();
        term.echo(false)?;
        term.query_cursor();
        term.buffering(false)?;
        io::stdout().flush()?;
        Ok(term)
    }

    pub fn release(&mut self) -> ResultBox<()> {
        self.buffering(true)?;
        self.echo(true)?;
        if let Some(ref cursor) = self.initial_cursor.clone() {
            self.move_cursor(cursor.x + 1, cursor.y + 1);
        }
        self.rmcup();
        io::stdout().flush()?;
        Ok(())
    }

    pub fn initial_cursor(&mut self, cursor: &Cursor) {
        if self.initial_cursor.is_none() {
            self.initial_cursor = Some((*cursor).clone());
            self.smcup();
        }
    }

    pub fn consume_output_buffer(&mut self) -> io::Result<()> {
        self.output.consume()
    }

    pub fn color(&mut self, b: &Option<Brush>) {
        self.output.write(&Brush::change(&self.brush, b));
    }

    /// Move cursor to the coordinate.
    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.write(&format!("\u{1b}[{};{}f", y + 1, x + 1));
    }

    /// If b is `true` then show the cursor. Otherwise hide.
    pub fn show_cursor(&mut self, b: bool) {
        if b {
            self.write(&String::from("\u{1b}[?25h"));
        } else {
            self.write(&String::from("\u{1b}[?25l"));
        }
    }

    pub fn write(&mut self, s: &String) {
        if self.buffering {
            io::stdout().write(s.as_bytes()).unwrap();
        } else {
            self.output.write(s);
        }
    }

    /// Draw ui::Line after the cursor.
    pub fn write_ui_line(&mut self, l: &Line) {
        for c in &l.chars {
            let prev = Some(c.brush.clone());
            if self.brush != prev {
                self.color(&prev);
                self.brush = prev;
            }
            // TODO: Optimize
            self.write(&c.chr.to_string());
        }
    }

    /// Draw ui::Char after the cursor.
    pub fn write_ui_char(&mut self, c: &Char) {
        if self.brush != Some(c.brush.clone()) {
            let br = self.brush.clone();
            self.color(&br);
            self.brush = Some(c.brush.clone());
        }
        // TODO: Optimize
        self.write(&c.chr.to_string());
    }

    /// Draw ui::Buffer at the coordinate.
    pub fn write_ui_buffer(&mut self, x: usize, y: usize, rect: &Rect) {
        self.move_cursor(x, y);
        for (i, l) in rect.lines.iter().enumerate() {
            self.move_cursor(x, y + i);
            self.write_ui_line(&l);
        }
    }

    pub fn get_size(&self) -> Result<(usize, usize)> {
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

    pub fn read(&self, limit: usize) -> ResultBox<String> {
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
        Ok(String::from_utf8(buf)?)
    }

    fn buffering(&mut self, on: bool) -> Result<()> {
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

    fn echo(&mut self, on: bool) -> Result<()> {
        unsafe {
            libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
            let mut termios = mem::uninitialized();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut termios) == -1 {
                return Err(Error::Tcgetattr);
            }
            if on {
                termios.c_lflag |= libc::ICANON;
                termios.c_lflag |= libc::ECHO;
                termios.c_iflag |= libc::ICRNL;
                termios.c_lflag |= libc::ISIG;
            } else {
                termios.c_lflag &= !libc::ICANON;
                termios.c_lflag &= !libc::ECHO;
                termios.c_iflag &= !libc::ICRNL;
                termios.c_lflag &= !libc::ISIG;
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
