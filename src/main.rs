extern crate libc;
mod io;
mod ui;

fn main() {
    io::init().unwrap();
    let chan = ui::handler::launch();
    let mut ev = io::Event::new().unwrap();
    ev.init().unwrap();
    ev.handle(chan).unwrap();
}
