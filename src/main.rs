#![feature(conservative_impl_trait)]

#[macro_use]
extern crate libc;

#[macro_use]
mod util;
mod buf;
mod io;
mod ui;
mod hq;
mod msg;

use ui::Ui;
use std::fs::File;
use std::io::Write;

fn main() {
    let ui = Ui::new().unwrap();
    let mut io = io::Handler::new(ui).unwrap();
    match io.run() {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log/main.log").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
