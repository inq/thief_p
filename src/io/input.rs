use std::error;
use std::fmt;
use libc;

def_error! {
    Read: "read returned -1",
    FGetfl: "fcntl(F_GETFL) returned -1",
    FSetfl: "fcntl(F_SETFL) returned -1",
}

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
