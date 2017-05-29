#![feature(proc_macro)]

extern crate libc;
#[macro_use]
extern crate proc_macros;

#[macro_use]
mod util;
mod buf;
mod io;
mod ui;
mod hq;
mod msg;
mod term;

use std::fs::File;
use std::io::Write;

/// The main function
/// ( io => hq => ui )
fn main() {
    let ui = ui::Ui::new().unwrap();
    let hq = hq::Hq::new(ui).unwrap();
    let mut io = io::Handler::new(hq).unwrap();
    match io.run() {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log/main.log").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
