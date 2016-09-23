use std::error;
use std::fmt;
use libc;

pub fn init() -> Result<(), Error> {
    let res = unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN)
    };
    if res == libc::SIG_ERR {
        return Err(Error::SigErr);
    }
    Ok(())
}


#[derive(Debug)]
pub enum Error {
    SigErr,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SigErr => "signal returned -1",
        }
    }
}
