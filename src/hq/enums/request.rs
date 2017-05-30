use hq;
use ui;
use std::str;
use term;

pub enum Request {
    Keyboard(term::Key),
    Resize(usize, usize),
    Single(usize),
    Pair(usize, usize),
}

impl Request {
    /// Convert to ui::Request
    pub fn to_ui(self) -> ui::Request {
        match self {
            Request::Keyboard(k) => ui::Request::Keyboard(k),
            Request::Resize(w, h) => ui::Request::Resize(w, h),
            Request::Single(x) => ui::Request::Single(x),
            Request::Pair(x, y) => ui::Request::Pair(x, y),
        }
    }

    /// Convert some events into readable format.
    pub fn normalize(self) -> Request {
        match self {
            Request::Single(1) => Request::Keyboard(term::Key::Home),
            Request::Single(4) => Request::Keyboard(term::Key::End),
            Request::Keyboard(k) => Request::Keyboard(term::Key::normalize(k)),
            etc => etc,
        }
    }

    pub fn from_char(c: char) -> Request {
        Request::Keyboard(if c as u32 <= 26 {
                              term::Key::Ctrl((c as u8 + b'a' - 1) as char)
                          } else {
                              term::Key::Char(c)
                          })
    }

    pub fn from_string(s: &str) -> (Option<Request>, String) {
        let mut it = s.chars();
        let res = match it.next() {
            Some('\x1b') => {
                match it.next() {
                    Some('[') => process_csi(&mut it),
                    _ => None,
                }
            }
            Some(c) => Some(Request::from_char(c)),
            _ => None,
        };
        (res.map(Request::normalize), it.collect())
    }
}

/// Read integer characters with termination symbol.
#[inline]
fn read_num(s: &mut str::Chars, seed: usize) -> Option<(usize, char)> {
    let mut acc = seed;
    for c in s {
        if c >= '0' && c <= '9' {
            acc = acc * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            return Some((acc, c));
        }
    }
    None
}

/// Try to read `CSIx;yR`.
#[inline]
fn check_num(it: &mut str::Chars, seed: usize) -> Option<Request> {
    match read_num(it, seed) {
        Some((y, ';')) => {
            if let Some((x, 'R')) = read_num(it, seed) {
                Some(Request::Pair(x, y))
            } else {
                None
            }
        }
        Some((n, '~')) => Some(Request::Single(n)),
        _ => None,
    }
}

/// After check
#[inline]
fn process_csi(s: &mut str::Chars) -> Option<hq::Request> {
    use hq::Request::Keyboard;
    if let Some(c) = s.next() {
        match c {
            '0'...'9' => check_num(s, c.to_digit(10).unwrap() as usize),
            'A' => Some(Keyboard(term::Key::Up)),
            'B' => Some(Keyboard(term::Key::Down)),
            'C' => Some(Keyboard(term::Key::Right)),
            'D' => Some(Keyboard(term::Key::Left)),
            _ => Some(Keyboard(term::Key::Meta(c as char))),
        }
    } else {
        Some(Keyboard(term::Key::Esc))
    }
}

#[test]
fn test_read_num() {
    let correct = String::from("1234;");
    assert_eq!(read_num(&mut correct.chars(), 0), Some((1234, ';')));
}
