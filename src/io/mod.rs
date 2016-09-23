use std::error::{Error};

mod handler;
mod term;
mod input;

pub use io::handler::{Handler};

pub fn init() -> Result<(), Box<Error>> {
    try!(input::init());
    try!(term::init());
    Ok(())
}
