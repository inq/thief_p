use std::mem;
use util;

#[derive(Debug)]
pub struct Line {
    cache: String,
    dirty: bool,
    prevs: String,
    nexts: String,
    x: usize,
}

const BUFSIZE: usize = 80;

impl Default for Line {
    fn default() -> Line {
        Line {
            cache: String::new(),
            dirty: false,
            prevs: String::with_capacity(BUFSIZE),
            nexts: String::with_capacity(BUFSIZE),
            x: Default::default(),
        }
    }
}

impl Line {
    /// Construct from a string.
    pub fn new_from_str(str: &str) -> Line {
        let mut res: Line = Default::default();
        res.push_before(str);
        res
    }

    /// Return the terminal x of the cursor.
    #[inline]
    pub fn x(&self) -> usize {
        self.x
    }

    /// Refresh the cache and get it.
    pub fn as_str(&mut self) -> &String {
        if self.dirty {
            self.cache.clear();
            self.cache.push_str(&self.prevs);
            self.cache.push_str(
                &self.nexts.chars().rev().collect::<String>(),
            );
            self.dirty = false;
        }
        &self.cache
    }

    /// Fill data from string.
    #[inline]
    pub fn replace(&mut self, src: String, x: usize) -> String {
        let res = self.as_str().clone();
        self.cache = src;
        self.prevs.clear();
        self.nexts.clear();
        self.x = 0;
        let mut iter = self.cache.chars();
        while self.x < x {
            if let Some(c) = iter.next() {
                self.prevs.push(c);
                self.x += util::term_width(c);
            } else {
                break;
            }
        }
        for c in iter.rev() {
            self.nexts.push(c);
        }
        self.dirty = false;
        res
    }

    /// Break the line, and return the first line as string.
    /// The posterior one replaces self.
    pub fn break_line(&mut self) -> String {
        self.dirty = true;
        self.x = 0;
        mem::replace(&mut self.prevs, String::with_capacity(BUFSIZE))
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
        self.dirty = true;
        self.x += util::term_width(c);
        self.prevs.push(c);
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
            assert_ne!(c, '\n');
            self.x += util::term_width(c);;
            self.prevs.push(c);
            true
        } else {
            false
        }
    }

    /// Prepend a line to this.
    pub fn prepend(&mut self, target: String) {
        let target = mem::replace(&mut self.prevs, target);
        self.prevs.push_str(&target);
        self.dirty = true;
        self.x = self.prevs
            .chars()
            .map({
                |c| util::term_width(c)
            })
            .sum();
    }

    /// Append a line to this.
    pub fn append(&mut self, target: String) {
        self.nexts.push_str(
            &target.chars().rev().collect::<String>(),
        );
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
        self.dirty = true;
        res
    }

    /// Append string after cursor.
    #[cfg(test)]
    pub fn push_after(&mut self, str: &str) {
        let reversed = str.chars().rev().collect::<String>();
        self.nexts.push_str(&reversed);
    }

    /// Convert to a string.
    /// This can be used for the debugging purpose.
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
