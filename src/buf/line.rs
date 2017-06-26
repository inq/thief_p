use std::mem;
use term;
use util;

#[derive(Debug)]
pub struct Line {
    cache: term::String,
    dirty: bool,
    prevs: term::String,
    nexts: term::String,
    x: usize,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            cache: term::String::new(),
            dirty: false,
            prevs: term::String::new(),
            nexts: term::String::new(),
            x: Default::default(),
        }
    }
}

impl Line {
    /// Construct from a string.
    pub fn new_from_string(value: term::String) -> Line {
        let mut res: Line = Default::default();
        res.push_before(value);
        res
    }

    /// Return the terminal x of the cursor.
    #[inline]
    pub fn x(&self) -> usize {
        self.x
    }

    /// Refresh the cache and get it.
    pub fn as_string(&mut self) -> &term::String {
        if self.dirty {
            self.cache.clear();
            self.cache.push_string(&mut self.prevs.clone());
            self.cache.push_string(&mut self.nexts.reversed());
            self.dirty = false;
        }
        &self.cache
    }

    /// Fill data from string.
    pub fn replace(&mut self, src: term::String, x: usize) -> term::String {
        let res = self.as_string().clone();
        self.cache = src;
        self.prevs.clear();
        self.nexts.clear();
        self.x = 0;
        let mut iter = self.cache.iter();
        while self.x < x {
            if let Some(c) = iter.next() {
                self.prevs.push(c.clone());
                self.x += c.width();
            } else {
                break;
            }
        }
        for c in iter.rev() {
            self.nexts.push(c.clone());
        }
        self.dirty = false;
        res
    }

    /// Break the line, and return the first line as string.
    /// The posterior one replaces self.
    pub fn break_line(&mut self) -> term::String {
        self.dirty = true;
        self.x = 0;
        mem::replace(&mut self.prevs, term::String::new())
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
    pub fn after_cursor(&self, limit: usize) -> term::String {
        self.nexts.reversed().take(limit)
    }

    /// Insert a char.
    pub fn insert(&mut self, c: char) {
        self.dirty = true;
        self.x += util::term_width(c);
        self.prevs.push(
            term::Char::new(c, term::Brush::black_and_white()),
        );
    }

    /// Append string before cursor.
    pub fn push_before(&mut self, mut value: term::String) {
        self.x += value.iter().map(|c| c.width()).sum();
        self.prevs.push_string(&mut value);
    }

    /// Move cursor left by 1 character.
    pub fn move_left(&mut self) -> bool {
        if let Some(c) = self.prevs.pop() {
            self.x -= c.width();
            self.nexts.push(c);
            true
        } else {
            false
        }
    }

    /// Move cursor right by 1 character.
    pub fn move_right(&mut self) -> bool {
        if let Some(c) = self.nexts.pop() {
            self.x += c.width();
            self.prevs.push(c);
            true
        } else {
            false
        }
    }

    /// Prepend a line to this.
    pub fn prepend(&mut self, target: term::String) {
        let mut target = mem::replace(&mut self.prevs, target);
        self.prevs.push_string(&mut target);
        self.dirty = true;
        self.x = self.prevs.iter().map(|c| c.width()).sum()
    }

    /// Append a line to this.
    pub fn append(&mut self, target: term::String) {
        self.nexts.push_string(&mut target.reversed());
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
            self.x += c.width();
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

    /// Convert to a string.
    /// This can be used for the debugging purpose.
    #[cfg(test)]
    pub fn to_str(&self) -> String {
        let mut res = String::from(self.prevs.to_str());
        res.push_str(&self.nexts.reversed().to_str());
        res
    }
}
