use std::error::{self, Error};
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum Event {
    Char{ c: char }
}

#[derive(Debug)]
pub enum ParseEventError {
    ParseError,
}

impl fmt::Display for ParseEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl error::Error for ParseEventError {
    fn description(&self) -> &str {
        match *self {
            ParseEventError::ParseError => "parse error"
        }
    }
}

impl str::FromStr for Event {
    type Err = ParseEventError;

    fn from_str(s: &str) -> Result<Event, ParseEventError> {
        match s {
            "c" => Ok(Event::Char { c: 'c' }),
            _ => Err(ParseEventError::ParseError),
        }
    }
}
