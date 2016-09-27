pub mod handler;
mod comp;
mod editor;
mod screen;
mod window;
mod res;

use libc;

pub use ui::res::*;

pub fn init() {
    unsafe {
        libc::setlocale(libc::LC_CTYPE, "".as_ptr() as *const i8);
    }
}
