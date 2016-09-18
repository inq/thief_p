extern crate libc;
mod io;

fn main() {
    io::input::nonblock_init().unwrap();
    let mut ev = io::event::Event::new().unwrap();
    ev.init().unwrap();
    ev.handle(|e| {
        let mut buf = Vec::with_capacity(256);;
        println!("{}", e.ident);
        try!(io::input::read(&mut buf).map_err(|e| e.to_string()));
        let s = try!(String::from_utf8(buf).map_err(|e| e.to_string()));
        println!("{}", s);
        Ok(())
    }).unwrap();
}
