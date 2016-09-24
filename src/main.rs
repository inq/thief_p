extern crate libc;
mod io;
mod ui;

use std::fs::File;
use std::io::Write;

fn main() {
    ui::init();
    let (w, h) = io::init().unwrap();
    let (a, b) = ui::handler::launch(w, h);
    let mut ev = io::Handler::new().unwrap();
    ev.init().unwrap();
    match ev.handle(a, b) {
        Ok(()) => (),
        Err(e) => {
            let mut f = File::create("log.txt").unwrap();
            f.write_all(e.description().as_bytes()).unwrap();
        }
    }
}
