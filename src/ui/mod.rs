pub mod handler;
mod buffer;
mod char;
mod color;
mod screen;
mod term;
mod line;

use libc;

pub fn init() {
    unsafe {
        libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
    }
}
