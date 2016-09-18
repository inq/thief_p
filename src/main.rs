extern crate libc;
mod io;
mod ui;

use ui::event::Event;
use std::str::FromStr;

fn main() {
    io::input::nonblock_init().unwrap();
    let mut ev = io::event::Event::new().unwrap();
    let chan = ui::handler::launch();
    ev.init().unwrap();
    loop {
        ev.handle(|_| {
            let mut buf = Vec::with_capacity(256);;
            try!(io::input::read(&mut buf));
            let s = try!(String::from_utf8(buf));
            let e = try!(Event::from_str(&s));
            try!(chan.send(e));
            Ok(())
        }).unwrap();
    }
}
