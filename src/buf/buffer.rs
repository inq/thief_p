use std::io::{BufReader, BufRead};
use std::{fs, mem, path};
use super::line::Line;
use util::ResultBox;
use msg;

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

pub enum BackspaceRes {
    Normal(String),
    PrevLine(msg::Pair),
    Unchanged,
}

pub enum KillLineRes {
    Normal,
    Empty(msg::Pair),
    Unchanged,
}

impl Buffer {
    /// Return the ith element.
    pub fn get(&self, i: usize) -> Option<&Line> {
        if i == self.prevs.len() {
            Some(&self.cur)
        } else if i < self.prevs.len() {
            Some(&self.prevs[i])
        } else if self.nexts.len() + self.prevs.len() > i {
            self.nexts.get(self.nexts.len() + self.prevs.len() - i)
        } else {
            None
        }
    }

    /// Return the total number of lines.
    pub fn get_line_num(&self) -> usize {
        1 + self.prevs.len() + self.nexts.len()
    }

    /// Get the x position of the cursor.
    #[inline]
    pub fn get_x(&self) -> usize {
        self.cur.get_x()
    }

    /// Get the y position of the cursor.
    #[inline]
    pub fn get_y(&self) -> usize {
        self.prevs.len()
    }

    /// Get the position of the cursor.
    #[inline]
    pub fn get_xy(&self) -> msg::Pair {
        msg::Pair {
            x: self.get_x(),
            y: self.get_y(),
        }
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

    /// Move up the cursor.
    #[inline]
    fn move_up(&mut self, offset: usize) {
        if let Some(l) = self.prevs.pop() {
            self.nexts.push(mem::replace(&mut self.cur, l));
            self.cur.set_cursor(offset);
        }
    }

    /// Move down the cursor.
    #[inline]
    fn move_down(&mut self, offset: usize) {
        if let Some(l) = self.nexts.pop() {
            self.prevs.push(mem::replace(&mut self.cur, l));
            self.cur.set_cursor(offset);
        }
    }

    /// Move to the beginning of the line.
    #[inline]
    pub fn move_begin_of_line(&mut self) -> msg::Pair {
        self.cur.move_begin();
        self.get_xy()
    }

    /// Move to the end of the line
    #[inline]
    pub fn move_end_of_line(&mut self) -> msg::Pair {
        self.cur.move_end();
        self.get_xy()
    }

    /// Break the line at the location of the cursor.
    #[inline]
    pub fn break_line(&mut self) -> msg::Pair {
        self.prevs.push(self.cur.break_line());
        self.get_xy()
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
            KillLineRes::Empty(self.get_xy())
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
            self.x = self.cur.get_x();
            BackspaceRes::PrevLine(self.get_xy())
        } else {
            BackspaceRes::Unchanged
        }
    }

    /// Move cursor.
    pub fn move_cursor(&mut self, dx: i8, dy: i8) -> msg::Pair {
        if dx != 0 {
            if dx > 0 {
                if !self.cur.move_right() {
                    self.move_down(0);
                }
            } else if !self.cur.move_left() {
                self.move_up(usize::max_value());
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
        msg::Pair {
            x: self.get_x(),
            y: self.get_y(),
        }
    }

    /// Read characters after cursor.
    #[inline]
    fn after_cursor(&self, limit: usize) -> String {
        self.cur.after_cursor(limit)
    }

    /// Insert a char at the location of the cursur.
    pub fn insert(&mut self, c: char, limit: usize) -> String {
        self.cur.insert(c);
        self.x = self.get_x();
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
        fs::File::open("Cargo.toml").unwrap().read_to_string(&mut a).unwrap();
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
