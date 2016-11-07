use std::io::{BufReader, BufRead, Read};
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

    /// Construct a buffer from a file.
    pub fn from_file<S: AsRef<Path> + ?Sized>(s: &S) -> ResultBox<Buffer> {
        let f = try!(File::open(s));
        let mut res = Buffer::new();
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

    /// Convert to a string.
    /// This can be used for the debugging purpose.
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
    let b = Buffer::from_file("Cargo.toml").unwrap().to_string();
    assert_eq!(a, b);
}
