use std::error::{self, Error};
use std::fmt;
use libc;

#[derive(Debug)]
pub enum InputError {
    ReadError,
    FGetflError,
    FSetflError,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl error::Error for InputError {
    fn description(&self) -> &str {
        match *self {
            InputError::ReadError => "read returned -1",
            InputError::FGetflError => "fcntl(F_GETFL) returned -1",
            InputError::FSetflError => "fcntl(F_SETFL) returned -1",
        }
    }
}

pub fn read(buf: &mut Vec<u8>) -> Result<isize, InputError> {
    let res = unsafe {
        libc::read(
            libc::STDIN_FILENO,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.capacity() as usize)
    };
    if res < 0 {
        return Err(InputError::ReadError);
    }
    unsafe {
        buf.set_len(res as usize);
    }
    Ok(res)
}

pub fn nonblock_init() -> Result<(), InputError> {
    let prev = unsafe {
        libc::fcntl(libc::STDIN_FILENO, libc::F_GETFL)
    };
    if prev == -1 {
        return Err(InputError::FGetflError);
    }

    let res = unsafe {
        libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, prev | libc::O_NONBLOCK)
    };
    if res == -1 {
        return Err(InputError::FSetflError);
    }
    Ok(())
}
