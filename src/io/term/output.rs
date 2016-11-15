use std::io::{self, Write};

pub struct Output {
    buffer: String,
    offset: usize,
}

impl Default for Output {
    fn default() -> Output {
        Output {
            buffer: String::with_capacity(4096),
            offset: Default::default(),
        }
    }
}

impl Output {
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.offset = 0;
    }

    pub fn consume(&mut self) -> io::Result<()> {
        if self.buffer.len() > self.offset {
            let (_, remaining) = self.buffer.split_at(self.offset);
            let offset = io::stdout().write(remaining.as_bytes()).unwrap();
            self.offset += offset;
        }
        if self.offset == self.buffer.len() {
            io::stdout().flush()?;
            self.clear();
        }
        Ok(())
    }

    pub fn write(&mut self, s: &String) {
        self.buffer.push_str(s);
    }
}
