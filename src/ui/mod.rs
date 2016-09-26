pub mod handler;
mod comp;
mod prim;
mod editor;
mod screen;
mod window;

use libc;

pub fn init() {
    unsafe {
        libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
    }
}
