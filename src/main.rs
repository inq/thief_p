extern crate libc;
mod io;

fn main() {
    io::input::nonblock_init().unwrap();
    let mut ev = io::event::Event::new().unwrap();
    ev.init().unwrap();
    ev.handle(|e| {
        let mut buf = Vec::with_capacity(256);;
        println!("{}", e.ident);
        try!(io::input::read(&mut buf));
        println!("{:?}", buf);
        Ok(())
    }).unwrap();
}
