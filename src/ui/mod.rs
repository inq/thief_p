pub mod handler;
mod buffer;
mod char;
mod color;
mod screen;
mod term;
mod line;

use libc;
use std::ffi::CStr;

pub fn init() {
    term::smcup();

    unsafe {
        libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
    }
    println!("{:?}", term::get_size());
}
