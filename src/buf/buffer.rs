use std::io::{BufReader, BufRead};
use std::slice::{Iter};
use std::iter::{Chain, Rev};
use std::fs::File;
use std::path::Path;
use super::line::Line;
use util::ResultBox;

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
fn buffer_from_file() {
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
    assert_eq!(buf.get_line(3).to_string().len(), 68);
}
