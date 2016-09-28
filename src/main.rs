extern crate libc;
extern crate regex;

#[macro_use]
mod error;

mod io;
mod ui;
mod util;

use ui::Ui;
use std::fs::File;
use std::io::Write;

fn main() {
    let ui = Ui::init();
    let mut io = io::Handler::init(ui).unwrap();
    match io.run() {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log/main.log").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
