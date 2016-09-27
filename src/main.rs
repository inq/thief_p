extern crate libc;

#[macro_use]
mod error;

mod io;
mod ui;

use std::fs::File;
use std::io::Write;

fn main() {
    ui::init();
    let (w, h) = io::init().unwrap();
    let (a, b) = ui::handler::launch(w, h);
    match io::run(a, b) {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log/main.log").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
