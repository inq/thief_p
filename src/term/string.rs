use std;
use term;

#[derive(Debug, Clone)]
pub struct String {
    vec: Vec<term::Char>,
}

const BUFSIZE: usize = 80;

impl String {
    pub fn new() -> Self {
        Self { vec: Vec::with_capacity(BUFSIZE) }
    }

    /// Consume self and return the vec element.
    pub fn take_vec(self) -> Vec<term::Char> {
        self.vec
    }

    /// Construct from std string with black & white color.
    pub fn from_std(ipt: &str, brush: term::Brush) -> Self {
        Self { vec: ipt.chars().map(|c| term::Char::new(c, brush)).collect() }
    }

    /// Skip first n characters
    /// TODO: Handle exception
    pub fn skip_n(mut self, n: usize) -> Self {
        Self { vec: self.vec.split_off(n) }
    }

    /// Length of the vector.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Empty the buffer.
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Append and element.
    pub fn push(&mut self, value: term::Char) {
        self.vec.push(value)
    }

    /// Append the string
    pub fn push_string(&mut self, value: &mut Self) {
        self.vec.append(&mut value.vec);
    }

    /// Remove the last element and returns it.
    pub fn pop(&mut self) -> Option<term::Char> {
        self.vec.pop()
    }

    /// Create a new reversed string.
    pub fn reversed(&self) -> Self {
        Self { vec: self.iter().rev().cloned().collect() }
    }

    /// Take only n characters.
    pub fn take(self, n: usize) -> Self {
        Self { vec: self.vec.into_iter().take(n).collect() }
    }

    /// Chars iterator.
    pub fn iter(&self) -> std::slice::Iter<term::Char> {
        self.vec.iter()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[cfg(test)]
    pub fn to_str(&self) -> std::string::String {
        self.iter().map(|c|c.chr).collect()
    }
}
