use std::error::Error;

mod event;
mod handler;
mod input;
mod term;

pub use io::handler::Handler;
pub use io::event::Event;

pub fn init() -> Result<(), Box<Error>> {
    try!(input::init());
    try!(term::init());
    Ok(())
}
