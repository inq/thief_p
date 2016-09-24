use std::{error, fmt, mem};
use libc;

pub fn init() -> Result<(usize, usize), Error> {
    let mut termios = unsafe { mem::uninitialized() };
    if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut termios) } == -1 {
        return Err(Error::Tcgetattr);
    }
    termios.c_lflag &= !(libc::ICANON);
    termios.c_lflag &= !(libc::ECHO);
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) } == -1 {
        return Err(Error::Tcsetattr);
    }

    smcup();
    get_size()
}

pub fn smcup() {
    print!("\u{1b}[?47h");
}

pub fn rmcup() {
    print!("\u{1b}[?47l");
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
    Tcgetattr,
    Tcsetattr,
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
            Error::Tcgetattr => "tcgetattr returned -1",
            Error::Tcsetattr => "tcsetattr returned -1",
            Error::Tiocgwinsz => "ioctl returned -1",
        }
    }
}
