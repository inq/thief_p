use std::error;
use std::io::{self, Write};

pub struct Output {
    buffer: String,
    offset: usize,
}

impl Output {
    pub fn new() -> Output {
        Output {
            buffer: String::with_capacity(4096),
            offset: 0,
        }
    }

    pub fn clear(&mut self) {
        self.offset = 0;
    }

    pub fn consume(&mut self) -> Result<(), Box<error::Error>> {
        if self.buffer.len() > self.offset {
            let (_, remaining) = self.buffer.split_at(self.offset);
            let offset = try!(io::stdout().write(remaining.as_bytes()));
            self.offset += offset;
            if self.offset == self.buffer.len() {
                try!(io::stdout().flush());
            }
        }
        Ok(())
    }

    pub fn write(&mut self, s: &String) {
        self.buffer.push_str(s);
    }
}
