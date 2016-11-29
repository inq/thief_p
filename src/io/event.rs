use std::char;
use std::str::Chars;

#[derive(Debug)]
pub enum Event {
    Char { c: char },
    Ctrl { c: char },
    Move { x: i8, y: i8 },
    Meta { c: char },
    Single { n: usize },
    Pair { x: usize, y: usize },
    Resize { w: usize, h: usize },
    Navigate { msg: String },
    Notify { s: String },
    OpenBuffer { s: String },
    Escape,
}

impl Event {
    pub fn from_char(c: char) -> Event {
        if c as u32 <= 26 {
            Event::Ctrl { c: (c as u8 + 'a' as u8 - 1) as char }
        } else {
            Event::Char { c: c }
        }
    }

    pub fn from_string(s: &String) -> (Option<Event>, String) {
        let mut it = s.chars();
        let res = match it.next() {
            Some('\x1b') => {
                if it.next() == Some('[') {
                    // CSI
                    process_csi(&mut it)
                } else {
                    None
                }
            }
            Some(c) => Some(Event::from_char(c)),
            _ => None,
        };
        (res, it.collect())
    }
}

/// Read integer characters with termination symbol.
#[inline]
fn read_num(s: &mut Chars, seed: usize) -> Option<(usize, char)> {
    let mut acc = seed;
    while let Some(c) = s.next() {
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
fn check_num(it: &mut Chars, seed: usize) -> Option<Event> {
    match read_num(it, seed) {
        Some((y, ';')) => {
            if let Some((x, 'R')) = read_num(it, seed) {
                Some(Event::Pair { x: x, y: y })
            } else {
                None
            }
        }
        Some((n, '~')) => Some(Event::Single { n: n }),
        _ => None,
    }
}

/// After check
#[inline]
fn process_csi(s: &mut Chars) -> Option<Event> {
    if let Some(c) = s.next() {
        match c {
            '0'...'9' => check_num(s, c.to_digit(10).unwrap() as usize),
            'A' => Some(Event::Move { x: 0, y: -1 }),
            'B' => Some(Event::Move { x: 0, y: 1 }),
            'C' => Some(Event::Move { x: 1, y: 0 }),
            'D' => Some(Event::Move { x: -1, y: 0 }),
            _ => Some(Event::Meta { c: c as char }),
        }
    } else {
        Some(Event::Escape)
    }
}

#[test]
fn test_read_num() {
    let correct = String::from("1234;");
    let wrong = String::from("a");
    assert_eq!(read_num(&mut correct.chars(), 0, ';'), Some(1234));
    assert_eq!(read_num(&mut correct.chars(), 0, '-'), None);
    assert_eq!(read_num(&mut wrong.chars(), 0, ';'), None);
}
