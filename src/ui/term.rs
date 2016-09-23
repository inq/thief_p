use std::mem;
use std::fmt;
use std::error;
use libc;

pub fn smcup() {
    print!("\u{1b}[?47h");
}

pub fn rmcup() {
    print!("\u{1b}[?47l");
}

pub fn get_size() -> Result<(u16, u16), Error> {
    let ws: libc::winsize = unsafe { mem::uninitialized() };
    let res = unsafe {
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &ws)
    };
    if res < 0 {
        return Err(Error::Tiocgwinsz);
    }
    Ok((ws.ws_col, ws.ws_row))
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
