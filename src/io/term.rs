use std::{error, fmt, mem};
use libc;

pub fn init() -> Result<(), Error> {
    let mut termios = unsafe { mem::uninitialized() };
    if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut termios) } == -1 {
        return Err(Error::Tcgetattr);
    }
    termios.c_lflag &= !(libc::ICANON);
    termios.c_lflag &= !(libc::ECHO);
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) } == -1 {
        return Err(Error::Tcsetattr);
    }

    Ok(())
}


#[derive(Debug)]
pub enum Error {
    Tcgetattr,
    Tcsetattr,
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
        }
    }
}
