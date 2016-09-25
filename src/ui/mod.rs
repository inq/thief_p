pub mod handler;
mod prim;
mod screen;
mod window;

use libc;

pub fn init() {
    unsafe {
        libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
    }
}
