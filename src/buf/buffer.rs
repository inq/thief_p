use std::io::{BufReader, BufRead, Read};
use std::slice::{Iter};
use std::iter::{Chain, Rev};
use std::fs::File;
use std::path::Path;
use super::line::Line;
use super::Direction;
use util::ResultBox;

/// Current line is the last element of the `after`.
pub struct Buffer {
    before: Vec<Line>,
    after: Vec<Line>,
}

const BUFSIZE: usize = 80;

impl Buffer {
    /// Construct a new empty buffer.
    pub fn new() -> Buffer {
        let mut res = Buffer {
            before: Vec::with_capacity(BUFSIZE),
            after: Vec::with_capacity(BUFSIZE),
        };
        res.after.push(Line::new());
        res
    }

    /// Construct a buffer from a file.
    pub fn from_file<S: AsRef<Path> + ?Sized>(s: &S) -> ResultBox<Buffer> {
        let mut res = Buffer {
            before: Vec::with_capacity(BUFSIZE),
            after: Vec::with_capacity(BUFSIZE),
        };
        let f = try!(File::open(s));
        let br = BufReader::new(&f);

        for line in br.lines() {
            if let Ok(s) = line {
                res.before.push(Line::from_string(&s));
            }
        }
        while let Some(l) = res.before.pop() {
            res.after.push(l);
        }
        Ok(res)
    }

    /// Move cursor.
    pub fn move_cursor(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Right => self.after[0].move_cursor(true),
            Direction::Left => self.after[0].move_cursor(false),
            _ => true,
        }
    }

    /// Insert a char at the location of the cursur.
    pub fn insert(&mut self, c: char) {
        let loc = self.after.len() - 1;
        self.after[loc].insert(c)
    }

    /// Break the line at the location of the cursor.
    pub fn break_line(&mut self) {
        let loc = self.after.len() - 1;
        let res = self.after[loc].break_line();
        self.before.push(res);
    }

    /// Iterate lines.
    pub fn iter(&self) -> Chain<Iter<Line>, Rev<Iter<Line>>> {
        self.before.iter().chain(self.after.iter().rev())
    }

    /// Convert to a string.
    /// This can be used for the debugging purpose.
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(1024);
        for ref v in &self.before {
            res.push_str(&v.to_string());
            res.push('\n');
        }
        for v in (0..self.after.len()).rev() {
            res.push_str(&self.after[v].to_string());
            res.push('\n');
        }
        res
    }
}

#[test]
fn test_buffer_from_file() {
    let mut a = String::with_capacity(1024);
    File::open("Cargo.toml").unwrap().read_to_string(&mut a).unwrap();
    let mut buf = Buffer::from_file("Cargo.toml").unwrap();
    assert_eq!(a, buf.to_string());
}

#[test]
fn test_get_line() {
    let buf = Buffer::from_file("LICENSE").unwrap();
    assert_eq!(buf.iter().nth(3).unwrap().to_string().len(), 68);
}

#[test]
fn test_insert() {
    let mut buf = Buffer::new();
    buf.insert('h');
    assert_eq!(buf.to_string(), "h\n");
}

#[test]
fn test_breakline() {
    let mut buf = Buffer::new();
    buf.break_line();
    assert_eq!(buf.to_string(), "\n\n");
    let mut buf = Buffer::new();
    buf.after[0] = Line::from_string(&String::from("Hello, world!"));
    buf.break_line();
    assert_eq!(buf.to_string(), "Hello, world!\n\n");
    let mut buf = Buffer::new();
    buf.after[0] = Line::from_string(&String::from("Hello, world!"));
    for _ in 0..5 {
        buf.after[0].move_cursor(false);
    }
    buf.break_line();
    assert_eq!(buf.to_string(), "Hello, w\norld!\n");
}
