extern crate libc;
mod io;
mod ui;

fn main() {
    ui::init();
    io::init().unwrap();
    let chan = ui::handler::launch();
    let mut ev = io::Handler::new().unwrap();
    ev.init().unwrap();
    ev.handle(chan).unwrap();
}
