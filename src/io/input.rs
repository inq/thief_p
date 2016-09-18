use libc;

pub fn read(buf: &mut [u8]) -> Result<isize, &'static str> {
    let res = unsafe {
        libc::read(
            libc::STDIN_FILENO,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len() as usize)
    };
    if res < 0 {
        Err("read returned -1")
    } else {
        Ok(res)
    }
}
