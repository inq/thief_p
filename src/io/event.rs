use std::char;
use std::str::Bytes;
use regex::Regex;

#[derive(Debug)]
pub enum Event {
    Char { c: char },
    Ctrl { c: char },
    Move { x: i8, y: i8 },
    Meta { c: char },
    Pair { x: usize, y: usize },
    Resize { w: usize, h: usize },
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
        match it.next() {
            Some('\u{1b}') => from_escape(&s),
            Some(c) => (Some(Event::from_char(c)), it.collect()),
            None => (None, it.collect()),
        }
    }
}

/// Check for CSI characters.
#[inline]
fn check_csi(s: &mut Bytes) -> bool {
    s.next() == Some(b'\x1b') && s.next() == Some(b'[')
}

/// Read integer characters with termination symbol.
#[inline]
fn read_num(s: &mut Bytes, d: u8) -> Option<usize> {
    let mut s = s.peekable();
    let mut acc = 0usize;
    while let Some(c) = s.next() {
        if c >= b'0' && c <= b'9' {
            acc = acc * 10 + (c - b'0') as usize;
        } else if c == d {
            return Some(acc)
        } else {
            return None
        }
    }
    None
}

/// Try to read `CSIx;yR`.
fn check_pair(s: &String) -> Option<(Event, String)> {
    let mut it = s.bytes();
    if !check_csi(&mut it) {
        return None
    }
    read_num(&mut it, b';').and_then(|x| {
        read_num(&mut it, b'R').map(|y| {
            (
                Event::Pair { x: x, y: y },
                String::from("")
            )
        })
    })
}

fn check_prefix(s: &String, t: &str) -> Option<String> {
    if s.starts_with(&t) {
        Some(s.chars().skip(t.len()).collect())
    } else {
        None
    }
}

fn from_escape(s: &String) -> (Option<Event>, String) {
    if let Some((e, r)) = check_pair(&s) {
        (Some(e), r)
    } else if let Some(r) = check_prefix(&s, "\u{1b}[A") {
        (Some(Event::Move { x: 0, y: -1 }), r)
    } else if let Some(r) = check_prefix(&s, "\u{1b}[B") {
        (Some(Event::Move { x: 0, y: 1 }), r)
    } else if let Some(r) = check_prefix(&s, "\u{1b}[C") {
        (Some(Event::Move { x: 1, y: 0 }), r)
    } else if let Some(r) = check_prefix(&s, "\u{1b}[D") {
        (Some(Event::Move { x: -1, y: 0 }), r)
    } else {
        let mut it = s.chars().skip(1);
        match it.next() {
            Some(k) => (Some(Event::Meta { c: k }), it.collect()),
            None => (Some(Event::Escape), it.collect()),
        }
    }
}

#[test]
fn test_check_csi() {
    let correct = String::from("\x1b[Hello");
    let wrong = String::from("[Hi]");
    assert!(check_csi(&mut correct.bytes()));
    assert!(!check_csi(&mut wrong.bytes()));
}

#[test]
fn test_read_num() {
    let correct = String::from("1234;");
    let wrong = String::from("a");
    assert_eq!(read_num(&mut correct.bytes(), b';'), Some(1234));
    assert_eq!(read_num(&mut correct.bytes(), b'-'), None);
    assert_eq!(read_num(&mut wrong.bytes(), b';'), None);
}

