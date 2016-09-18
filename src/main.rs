extern crate libc;
mod io;

fn main() {
    let mut buf = [0u8; 256];
    let len = io::input::read(&mut buf).unwrap();
    println!("read {} chars", len);
}
