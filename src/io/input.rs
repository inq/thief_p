use libc;

pub fn read(buf: &mut Vec<u8>) -> Result<isize, &'static str> {
    let res = unsafe {
        libc::read(
            libc::STDIN_FILENO,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.capacity() as usize)
    };
    if res < 0 {
        return Err("read returned -1");
    }
    unsafe {
        buf.set_len(res as usize);
    }
    Ok(res)
}

pub fn nonblock_init() -> Result<(), &'static str> {
    let prev = unsafe {
        libc::fcntl(libc::STDIN_FILENO, libc::F_GETFL)
    };
    if prev == -1 {
        return Err("fcntl(F_GETFL) returned -1");
    }

    let res = unsafe {
        libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, prev | libc::O_NONBLOCK)
    };
    if res == -1 {
        return Err("fcntl(F_SETFL) returned -1");
    }
    Ok(())
}
