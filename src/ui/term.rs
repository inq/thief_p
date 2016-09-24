use std::mem;
use std::fmt;
use std::error;
use libc;

pub fn smcup(buf: &mut String) {
    buf.push_str(&format!("\u{1b}[?47h"));
}

#[allow(dead_code)]
pub fn rmcup(buf: &mut String) {
    buf.push_str(&format!("\u{1b}[?47l"));
}

pub fn movexy(buf: &mut String, x: usize, y: usize) {
    buf.push_str(&format!("\u{1b}[{};{}f", y, x));
}

pub fn get_size() -> Result<(usize, usize), Error> {
    let ws: libc::winsize = unsafe { mem::uninitialized() };
    let res = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws) };
    if res < 0 {
        return Err(Error::Tiocgwinsz);
    }
    Ok((ws.ws_col as usize, ws.ws_row as usize))
}


#[derive(Debug)]
pub enum Error {
    Tiocgwinsz,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Tiocgwinsz => "ioctl returned -1",
        }
    }
}
