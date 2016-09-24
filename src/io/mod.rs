use std::error::Error;

mod event;
mod handler;
mod input;
mod signal;
mod term;

pub use io::handler::Handler;
pub use io::event::Event;

pub fn init() -> Result<(usize, usize), Box<Error>> {
    try!(signal::init());
    try!(input::init());
    let res = try!(term::init());
    Ok(res)
}
