use std::error;
use std::fmt;
use libc;

pub fn init() -> Result<(), Error> {
    let prev = unsafe { libc::fcntl(libc::STDIN_FILENO, libc::F_GETFL) };
    if prev == -1 {
        return Err(Error::FGetfl);
    }

    let res = unsafe { libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, prev | libc::O_NONBLOCK) };
    if res == -1 {
        return Err(Error::FSetfl);
    }
    Ok(())
}

pub fn read(buf: &mut Vec<u8>) -> Result<isize, Error> {
    let res = unsafe {
        libc::read(libc::STDIN_FILENO,
                   buf.as_mut_ptr() as *mut libc::c_void,
                   buf.capacity() as usize)
    };
    if res < 0 {
        return Err(Error::Read);
    }
    unsafe {
        buf.set_len(res as usize);
    }
    Ok(res)
}


#[derive(Debug)]
pub enum Error {
    Read,
    FGetfl,
    FSetfl,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Read => "read returned -1",
            Error::FGetfl => "fcntl(F_GETFL) returned -1",
            Error::FSetfl => "fcntl(F_SETFL) returned -1",
        }
    }
}
