use std::{error, fmt, mem};
use libc;

def_error! {
    Tcgetattr: "tcgetattr returned -1",
    Tcsetattr: "tcsetattr returned -1",
    Tiocgwinsz: "ioctl returned -1",
}

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
