use std::iter::{Chain, Rev};
use std::str::Chars;
use std::mem;
use util;

#[derive(Debug)]
pub struct Line {
    prevs: String,
    nexts: String,
    x: usize,
}

const BUFSIZE: usize = 80;

impl Default for Line {
    fn default() -> Line {
        Line {
            prevs: String::with_capacity(BUFSIZE),
            nexts: String::with_capacity(BUFSIZE),
            x: Default::default(),
        }
    }
}

impl Line {
    /// Return the terminal x of the cursor.
    pub fn get_x(&self) -> usize {
        self.x
    }

    /// Break the line.
    pub fn break_line(&mut self) -> Line {
        let res = mem::replace(&mut self.prevs, String::with_capacity(BUFSIZE));
        let x = res.len();
        self.x = 0;
        Line {
            prevs: res,
            nexts: String::with_capacity(BUFSIZE),
            x: x,
        }
    }

    /// Set the cursor position by the given x coordinate.
    pub fn set_cursor(&mut self, x: usize) {
        while self.prevs.len() > x {
            self.move_left();
        }
        while self.prevs.len() < x && !self.nexts.is_empty() {
            self.move_right();
        }
    }

    /// Get string after cursor
    pub fn after_cursor(&self, limit: usize) -> String {
        self.nexts.chars().rev().take(limit).collect()
    }

    /// Insert a char.
    pub fn insert(&mut self, c: char) {
        self.x += util::term_width(c);
        self.prevs.push(c);
    }

    /// Iterate chars.
    pub fn iter(&self) -> Chain<Chars, Rev<Chars>> {
        self.prevs.chars().chain(self.nexts.chars().rev())
    }

    /// Construct from a string.
    pub fn from_string(str: &str) -> Line {
        let mut res: Line = Default::default();
        res.push_before(str);
        res
    }

    /// Append string before cursor.
    pub fn push_before(&mut self, str: &str) {
        self.x += str.chars()
            .map({
                |c| util::term_width(c)
            })
            .sum();
        self.prevs.push_str(str);
    }

    /// Append string after cursor.
    #[allow(dead_code)]
    pub fn push_after(&mut self, str: &str) {
        let reversed = str.chars().rev().collect::<String>();
        self.nexts.push_str(&reversed);
    }

    /// Move cursor left by 1 character.
    pub fn move_left(&mut self) -> bool {
        if let Some(c) = self.prevs.pop() {
            self.x -= util::term_width(c);
            self.nexts.push(c);
            true
        } else {
            false
        }
    }

    /// Move cursor right by 1 character.
    pub fn move_right(&mut self) -> bool {
        if let Some(c) = self.nexts.pop() {
            self.x += util::term_width(c);;
            self.prevs.push(c);
            true
        } else {
            false
        }
    }

    /// Prepend a line to this.
    pub fn prepend(&mut self, mut target: Line) {
        mem::swap(&mut self.prevs, &mut target.prevs);
        self.prevs.push_str(&target.nexts.chars().rev().collect::<String>());
        self.x = self.prevs
            .chars()
            .map({
                |c| util::term_width(c)
            })
            .sum();
    }

    /// Append a line to this.
    pub fn append(&mut self, mut target: Line) {
        mem::swap(&mut self.nexts, &mut target.nexts);
        self.nexts.push_str(&target.prevs.chars().rev().collect::<String>());
    }

    /// Move to the begining of the line.
    #[inline]
    pub fn move_begin(&mut self) {
        while let Some(c) = self.prevs.pop() {
            self.nexts.push(c);
        }
        self.x = 0;
    }

    /// Move to the end of the line.
    #[inline]
    pub fn move_end(&mut self) {
        while let Some(c) = self.nexts.pop() {
            self.x += util::term_width(c);
            self.prevs.push(c);
        }
    }

    /// Delete single character before cursor.
    #[inline]
    pub fn backspace(&mut self) -> bool {
        self.prevs.pop().is_some()
    }

    /// Delete every characters after cursor. Return true iff there is any deleted character.
    #[inline]
    pub fn kill(&mut self) -> bool {
        let res = !self.nexts.is_empty();
        self.nexts.clear();
        res
    }

    /// Convert to a string.
    /// This can be used for the debugging purpose.
    #[cfg(test)]
    pub fn to_string(&self) -> String {
        let mut res = self.prevs.to_owned();
        res.push_str(&self.nexts.chars().rev().collect::<String>());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut t: Line = Default::default();
        t.push_before("hello");
        assert_eq!(t.to_string(), "hello");
        t.push_after("world");
        assert_eq!(t.to_string(), "helloworld");
    }

    #[test]
    fn test_move_cursor() {
        let mut t: Line = Default::default();
        t.push_before("hello");
        t.push_after("world");
        for _ in 0..5 {
            assert_eq!(t.move_right(), true);
        }
        assert_eq!(t.move_right(), false);
        assert_eq!(t.nexts, "");
        for _ in 0..10 {
            assert_eq!(t.move_left(), true);
        }
        assert_eq!(t.move_left(), false);
        assert_eq!(t.prevs, "");
    }

    #[test]
    fn test_insert() {
        let mut t: Line = Default::default();
        t.insert('h');
        assert_eq!(t.to_string(), "h");
        t.push_before("ell");
        t.push_after("world");
        t.insert('o');
        assert_eq!(t.to_string(), "helloworld");
    }

    #[test]
    fn test_break_line() {
        let mut t: Line = Default::default();
        t.push_before("hello");
        t.push_after("world");
        let u = t.break_line();
        assert_eq!(t.to_string(), "world");
        assert_eq!(u.to_string(), "hello");
    }
}
