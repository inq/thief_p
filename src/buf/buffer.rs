use std::io::{BufReader, BufRead, Read};
use std::slice::{Iter};
use std::iter::{Chain, Rev};
use std::fs::File;
use std::path::Path;
use super::line::Line;
use util::ResultBox;

/// Current line is the last element of the `before`.
pub struct Buffer {
    before: Vec<Line>,
    after: Vec<Line>,
}

const BUFSIZE: usize = 80;

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            before: Vec::with_capacity(BUFSIZE),
            after: Vec::with_capacity(BUFSIZE),
        }
    }

    /// Insert a char at the location of the cursur.
    pub fn insert(&mut self, c: char) {
        if self.before.len() == 0 {
            self.before.push(Line::new());
        }
        let loc = self.before.len() - 1;
        self.before[loc].insert(c)
    }

    /// Iterate lines.
    pub fn iter(&self) -> Chain<Iter<Line>, Rev<Iter<Line>>> {
        self.before.iter().chain(self.after.iter().rev())
    }

    /// Construct a buffer from a file.
    pub fn load_file<S: AsRef<Path> + ?Sized>(&mut self, s: &S) -> ResultBox<()> {
        let f = try!(File::open(s));
        let br = BufReader::new(&f);

        for line in br.lines() {
            if let Ok(s) = line {
                self.before.push(Line::from_string(&s));
            }
        }
        while let Some(l) = self.before.pop() {
            self.after.push(l);
        }
        Ok(())
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
    let mut buf = Buffer::new();
    buf.load_file("Cargo.toml").unwrap();
    assert_eq!(a, buf.to_string());
}

#[test]
fn test_get_line() {
    let mut buf = Buffer::new();
    buf.load_file("LICENSE").unwrap();
    assert_eq!(buf.iter().nth(3).unwrap().to_string().len(), 68);
}

#[test]
fn test_insert() {
    let mut buf = Buffer::new();
    buf.insert('h');
    assert_eq!(buf.to_string(), "h\n");
}
