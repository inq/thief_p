use std::{error, fmt, str};

#[derive(Debug)]
pub enum Event {
    Char{ c: char }
}

impl str::FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Event, Error> {
        match s {
            "c" => Ok(Event::Char { c: 'c' }),
            _ => Err(Error::Parse),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    Parse,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parse => "parse error"
        }
    }
}
