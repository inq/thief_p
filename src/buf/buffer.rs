use std::io::{BufReader, BufRead};
use std::{fs, mem, path};
use super::line::Line;
use util::ResultBox;

pub struct Buffer {
    cur: Line,
    x: usize,
    prevs: Vec<Line>,
    nexts: Vec<Line>,
}

const BUFSIZE: usize = 80;

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            cur: Default::default(),
            x: Default::default(),
            prevs: Vec::with_capacity(BUFSIZE),
            nexts: Vec::with_capacity(BUFSIZE),
        }
    }
}

impl Buffer {
    /// Return the ith element.
    pub fn get(&self, i: usize) -> Option<&Line> {
        if i == self.prevs.len() {
            Some(&self.cur)
        } else if i < self.prevs.len() {
            Some(&self.prevs[i])
        } else {
            self.nexts.get(self.nexts.len() + self.prevs.len() - i)
        }
    }

    /// Return the total number of lines.
    pub fn get_line_num(&self) -> usize {
        1 + self.prevs.len() + self.nexts.len()
    }

    /// Get the x position of the cursor.
    pub fn get_x(&self) -> usize {
        self.cur.get_x()
    }

    /// Get the y position of the cursor.
    pub fn get_y(&self) -> usize {
        self.prevs.len()
    }

    /// Construct a buffer from a file.
    pub fn from_file<S: AsRef<path::Path> + ?Sized>(s: &S) -> ResultBox<Buffer> {
        let f = fs::File::open(s)?;
        let br = BufReader::new(&f);
        let mut prevs = vec![];
        for line in br.lines() {
            if let Ok(s) = line {
                prevs.push(Line::from_string(&s));
            }
        }
        let cur = prevs.pop().unwrap_or_default();
        Ok(Buffer {
            prevs: prevs,
            cur: cur,
            ..Default::default()
        })
    }

    /// Read characters after cursor.
    pub fn after_cursor(&self, limit: usize) -> String {
        self.cur.after_cursor(limit)
    }

    /// Move up the cursor.
    fn move_up(&mut self, offset: usize) {
        if let Some(l) = self.prevs.pop() {
            self.nexts.push(mem::replace(&mut self.cur, l));
            self.cur.set_cursor(offset);
        }
    }

    /// Move down the cursor.
    fn move_down(&mut self, offset: usize) {
        if let Some(l) = self.nexts.pop() {
            self.prevs.push(mem::replace(&mut self.cur, l));
            self.cur.set_cursor(offset);
        }
    }

    /// Set the cursor by the given coordinate.
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        while self.prevs.len() > y {
            self.move_up(x);
        }
        while self.prevs.len() < y && self.prevs.len() > 0 {
            self.move_down(x);
        }
        self.cur.set_cursor(x);
    }

    /// Move cursor.
    pub fn move_cursor(&mut self, dx: i8, dy: i8) {
        if dx != 0 {
            if dx > 0 {
                if !self.cur.move_right() {
                    self.move_down(0);
                }
            } else {
                if !self.cur.move_left() {
                    self.move_up(usize::max_value());
                }
            }
            self.x = self.cur.get_x();
        }
        if dy != 0 {
            let x = self.x;
            if dy > 0 {
                self.move_down(x);
            } else {
                self.move_up(x);
            }
        }
    }

    /// Insert a char at the location of the cursur.
    pub fn insert(&mut self, c: char) {
        self.cur.insert(c)
    }

    /// Break the line at the location of the cursor.
    pub fn break_line(&mut self) {
        self.prevs.push(self.cur.break_line());
    }

    /// Convert to a string.
    /// This can be used for the debugging purpose.
    #[cfg(test)]
    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(1024);
        for ref v in &self.prevs {
            res.push_str(&v.to_string());
            res.push('\n');
        }
        res.push_str(&self.cur.to_string());
        res.push('\n');
        for v in self.nexts.iter().rev() {
            res.push_str(&v.to_string());
            res.push('\n');
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use buf::Line;
    use super::*;

    #[test]
    fn test_buffer_from_file() {
        let mut a = String::with_capacity(1024);
        fs::File::open("Cargo.toml").unwrap().read_to_string(&mut a).unwrap();
        let buf = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!(a, buf.to_string());
    }

    #[test]
    fn test_insert() {
        let mut buf: Buffer = Default::default();
        buf.insert('h');
        assert_eq!(buf.to_string(), "h\n");
    }

    #[test]
    fn test_breakline() {
        let mut buf: Buffer = Default::default();
        buf.break_line();
        assert_eq!(buf.to_string(), "\n\n");
        let mut buf =
            Buffer { cur: Line::from_string(&String::from("Hello, world!")), ..Default::default() };
        assert_eq!(buf.to_string(), "Hello, world!\n");
        buf.cur.set_cursor(usize::max_value());
        buf.break_line();
        assert_eq!(buf.to_string(), "Hello, world!\n\n");
        let mut buf =
            Buffer { cur: Line::from_string(&String::from("Hello, world!")), ..Default::default() };
        buf.cur.set_cursor(5);
        buf.break_line();
        assert_eq!(buf.to_string(), "Hello\n, world!\n");
    }
}
