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
        ev.handle(|e| {
            let mut buf = Vec::with_capacity(256);;
            println!("{}", e.ident);
            try!(io::input::read(&mut buf).map_err(|e| e.to_string()));
            let s = try!(String::from_utf8(buf).map_err(|e| e.to_string()));
            let e = try!(Event::from_str(&s).map_err(|e| e.to_string()));
            try!(chan.send(e).map_err(|e| e.to_string()));
            Ok(())
        }).unwrap();
    }
}
