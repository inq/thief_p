//#![feature(proc_macro)]

extern crate libc;
#[macro_use]
extern crate proc_macros;
extern crate syntect;

#[macro_use]
mod util;
mod buf;
mod io;
mod ui;
mod hq;
mod term;

use std::fs::File;
use std::io::Write;

/// The main function
/// `io::Handler` => `hq::Handler` => `ui::Screen`
fn main() {
    let screen = ui::Screen::new().unwrap();
    let hq_handler = hq::Handler::new(screen).unwrap();
    let mut io_handler = io::Handler::new(hq_handler).unwrap();
    match io_handler.run() {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log/main.log").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
