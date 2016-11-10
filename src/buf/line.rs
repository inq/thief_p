use std::iter::{Chain, Rev};
use std::str::Chars;

#[derive(Debug)]
pub struct Line {
    before: String,
    after: String,
}

const BUFSIZE: usize = 80;

impl Line {
    pub fn new() -> Line {
        Line {
            before: String::with_capacity(BUFSIZE),
            after: String::with_capacity(BUFSIZE),
        }
    }

    /// Iterate chars.
    pub fn iter(&self) -> Chain<Chars, Rev<Chars>> {
        self.before.chars().chain(self.after.chars().rev())
    }

    /// Construct from a string.
    pub fn from_string(str: &String) -> Line {
        let mut res = Line::new();
        res.append_before_cursor(str);
        res
    }

    /// Append string before cursor.
    pub fn append_before_cursor(&mut self, str: &String) {
        self.before.push_str(str);
    }

    /// Append string after cursor.
    #[allow(dead_code)]
    pub fn append_after_cursor(&mut self, str: &String) {
        let reversed = str.chars().rev().collect::<String>();
        self.after.push_str(&reversed);
    }

    /// Move cursor by 1 character.
    /// If `right` is `true`, then move to right direction. Otherwise,
    /// move to left direction. Returns `true` if succeed.
    #[allow(dead_code)]
    pub fn move_cursor(&mut self, right: bool) -> bool {
        let (from, to) = if right {
            (&mut self.after, &mut self.before)
        } else {
            (&mut self.before, &mut self.after)
        };
        match from.pop() {
            Some(c) => {
                to.push(c);
                true
            }
            None => false
        }
    }

    /// Convert to a string.
    /// This can be used for the debugging purpose.
    pub fn to_string(&self) -> String {
        let mut res = self.before.to_owned();
        res.push_str(&self.after.chars().rev().collect::<String>());
        res
    }
}

#[test]
fn basic_operations() {
    let mut t = Line::new();
    t.append_before_cursor(&String::from("hello"));
    assert_eq!(t.to_string(), "hello");
    t.append_after_cursor(&String::from("world"));
    assert_eq!(t.to_string(), "helloworld");
}

#[test]
fn move_cursor() {
    let mut t = Line::new();
    t.append_before_cursor(&String::from("hello"));
    t.append_after_cursor(&String::from("world"));
    for _ in 0..5 {
        assert_eq!(t.move_cursor(true), true);
    }
    assert_eq!(t.move_cursor(true), false);
    assert_eq!(t.after, "");
    for _ in 0..10 {
        assert_eq!(t.move_cursor(false), true);
    }
    assert_eq!(t.move_cursor(false), false);
    assert_eq!(t.before, "");
}
