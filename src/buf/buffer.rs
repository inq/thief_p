use hq;
use std::io::{BufReader, BufRead};
use std::{fs, path};
use util::ResultBox;
use buf::Line;

pub struct Buffer {
    cur: Line,
    x: usize,
    prevs: Vec<String>,
    nexts: Vec<String>,
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

pub enum BackspaceRes {
    Normal(String),
    PrevLine(hq::Pair),
    Unchanged,
}

pub enum KillLineRes {
    Normal,
    PullUp,
    Unchanged,
}

impl Buffer {
    /// Return the ith element.
    pub fn get(&mut self, i: usize) -> Option<&String> {
        if i == self.prevs.len() {
            Some(self.cur.as_str())
        } else if i < self.prevs.len() {
            Some(&self.prevs[i])
        } else if self.nexts.len() + self.prevs.len() >= i {
            self.nexts.get(self.nexts.len() + self.prevs.len() - i)
        } else {
            None
        }
    }

    pub fn cur(&self) -> &Line {
        &self.cur
    }

    /// Return the total number of lines.
    pub fn line_num(&self) -> usize {
        1 + self.prevs.len() + self.nexts.len()
    }

    /// Get the x position of the cursor.
    #[inline]
    pub fn x(&self) -> usize {
        self.cur.x()
    }

    /// Get the y position of the cursor.
    #[inline]
    pub fn y(&self) -> usize {
        self.prevs.len()
    }

    /// Get the position of the cursor.
    #[inline]
    pub fn cursor(&self) -> hq::Pair {
        (self.x(), self.y())
    }

    /// Construct a buffer from a file.
    pub fn from_file<S: AsRef<path::Path> + ?Sized>(s: &S) -> ResultBox<Buffer> {
        let f = fs::File::open(s)?;
        let br = BufReader::new(&f);
        let mut prevs = vec![];
        for line in br.lines() {
            if let Ok(s) = line {
                prevs.push(s);
            }
        }
        let cur = Line::new_from_str(&prevs.pop().unwrap_or_default());
        let mut buf = Buffer {
            prevs: prevs,
            cur: cur,
            ..Default::default()
        };
        buf.set_cursor(0, 0);
        Ok(buf)
    }

    /// Move up the cursor.
    #[inline]
    fn move_up(&mut self, offset: usize) {
        if let Some(s) = self.prevs.pop() {
            self.nexts.push(self.cur.replace(s, offset));
        }
    }

    /// Move down the cursor.
    #[inline]
    fn move_down(&mut self, offset: usize) {
        if let Some(s) = self.nexts.pop() {
            self.prevs.push(self.cur.replace(s, offset));
        }
    }

    /// Move to the beginning of the line.
    #[inline]
    pub fn move_begin_of_line(&mut self) -> hq::Pair {
        self.cur.move_begin();
        self.cursor()
    }

    /// Move to the end of the line
    #[inline]
    pub fn move_end_of_line(&mut self) -> hq::Pair {
        self.cur.move_end();
        self.cursor()
    }

    /// Break the line at the location of the cursor.
    #[inline]
    pub fn break_line(&mut self) -> hq::Pair {
        self.prevs.push(self.cur.break_line());
        self.x = 0;
        self.cursor()
    }

    /// Set the cursor by the given coordinate.
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        while self.prevs.len() > y {
            self.move_up(x);
        }
        while self.prevs.len() < y && !self.prevs.is_empty() {
            self.move_down(x);
        }
        self.cur.set_cursor(x);
    }

    /// Delete every characters after cursor.
    #[inline]
    pub fn kill_line(&mut self) -> KillLineRes {
        if self.cur.kill() {
            KillLineRes::Normal
        } else if let Some(line) = self.nexts.pop() {
            self.cur.append(line);
            KillLineRes::PullUp
        } else {
            KillLineRes::Unchanged
        }
    }

    /// Backspace.
    pub fn backspace(&mut self, limit: usize) -> BackspaceRes {
        if self.cur.backspace() {
            BackspaceRes::Normal(self.after_cursor(limit))
        } else if let Some(line) = self.prevs.pop() {
            self.cur.prepend(line);
            self.x = self.cur.x();
            BackspaceRes::PrevLine(self.cursor())
        } else {
            BackspaceRes::Unchanged
        }
    }

    /// Move cursor.
    pub fn move_cursor(&mut self, dx: i8, dy: i8) -> hq::Pair {
        if dx != 0 {
            if dx > 0 {
                if !self.cur.move_right() {
                    self.move_down(0);
                }
            } else if !self.cur.move_left() {
                self.move_up(usize::max_value());
            }
            self.x = self.cur.x();
        }
        if dy != 0 {
            let x = self.x;
            if dy > 0 {
                self.move_down(x);
            } else {
                self.move_up(x);
            }
        }
        self.cursor()
    }

    /// Read characters after cursor.
    #[inline]
    fn after_cursor(&self, limit: usize) -> String {
        self.cur.after_cursor(limit)
    }

    /// Insert a char at the location of the cursur.
    pub fn insert(&mut self, c: char, limit: usize) -> String {
        self.cur.insert(c);
        self.x = self.x();
        self.after_cursor(limit)
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
        fs::File::open("Cargo.toml")
            .unwrap()
            .read_to_string(&mut a)
            .unwrap();
        let buf = Buffer::from_file("Cargo.toml").unwrap();
        assert_eq!(a, buf.to_string());
    }

    #[test]
    fn test_insert() {
        let mut buf: Buffer = Default::default();
        buf.insert('h', 10);
        assert_eq!(buf.to_string(), "h\n");
    }

    #[test]
    fn test_breakline() {
        let mut buf: Buffer = Default::default();
        buf.break_line();
        assert_eq!(buf.to_string(), "\n\n");
        let mut buf = Buffer {
            cur: Line::new_from_str(&"Hello, world!"),
            ..Default::default()
        };
        assert_eq!(buf.to_string(), "Hello, world!\n");
        buf.cur.set_cursor(usize::max_value());
        buf.break_line();
        assert_eq!(buf.to_string(), "Hello, world!\n\n");
        let mut buf = Buffer {
            cur: Line::new_from_str(&"Hello, world!"),
            ..Default::default()
        };
        buf.cur.set_cursor(5);
        buf.break_line();
        assert_eq!(buf.to_string(), "Hello\n, world!\n");
    }
}
