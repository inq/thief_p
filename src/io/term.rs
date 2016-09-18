use std::error::{self, Error};
use std::fmt;
use std::mem;
use libc;

#[derive(Debug)]
pub enum TermError {
    TcgetattrError,
    TcsetattrError,
}

impl fmt::Display for TermError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl error::Error for TermError {
    fn description(&self) -> &str {
        match *self {
            TermError::TcgetattrError => "tcgetattr returned -1",
            TermError::TcsetattrError => "tcsetattr returned -1",
        }
    }
}

pub fn init() -> Result<(), TermError> {
    let mut termios = unsafe { mem::uninitialized() };
    if unsafe { libc::tcgetattr(0, &mut termios) } == -1 {
        return Err(TermError::TcgetattrError);
    }
    termios.c_lflag &= !(libc::ICANON);
    termios.c_lflag &= !(libc::ECHO);
    if unsafe { libc::tcsetattr(0, libc::TCSANOW, &termios) } == -1 {
        return Err(TermError::TcsetattrError);
    }
    Ok(())
}
