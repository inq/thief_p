#[macro_use]
extern crate libc;
extern crate regex;

#[macro_use]
mod util;
mod io;
mod ui;

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
