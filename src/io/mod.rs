use std::error::{Error};

mod event;
mod term;
mod input;

pub use io::event::{Event};

pub fn init() -> Result<(), Box<Error>> {
    try!(input::init());
    try!(term::init());
    Ok(())
}
