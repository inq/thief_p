use std::char;
use std::str::Chars;
use common::Key;

#[derive(Clone, Debug)]
pub enum Event {
    Keyboard(Key),
    Single(usize),
    Pair(usize, usize),
    Resize(usize, usize),
    Navigate(String),
    Notify(String),
    OpenBuffer(String),
}

impl Event {
    /// Convert some events into readable format.
    pub fn normalize(self) -> Event {
        match self {
            Event::Single(1) => Event::Keyboard(Key::Home),
            Event::Single(4) => Event::Keyboard(Key::End),
            Event::Keyboard(k) => Event::Keyboard(Key::normalize(k)),
            etc => etc,
        }
    }

    pub fn from_char(c: char) -> Event {
        Event::Keyboard(if c as u32 <= 26 {
            Key::Ctrl((c as u8 + 'a' as u8 - 1) as char)
        } else {
            Key::Char(c)
        })
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
        (res.map(Event::normalize), it.collect())
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
                Some(Event::Pair(x, y))
            } else {
                None
            }
        }
        Some((n, '~')) => Some(Event::Single(n)),
        _ => None,
    }
}

/// After check
#[inline]
fn process_csi(s: &mut Chars) -> Option<Event> {
    if let Some(c) = s.next() {
        match c {
            '0'...'9' => check_num(s, c.to_digit(10).unwrap() as usize),
            'A' => Some(Event::Keyboard(Key::Up)),
            'B' => Some(Event::Keyboard(Key::Down)),
            'C' => Some(Event::Keyboard(Key::Right)),
            'D' => Some(Event::Keyboard(Key::Left)),
            _ => Some(Event::Keyboard(Key::Meta(c as char))),
        }
    } else {
        Some(Event::Keyboard(Key::Esc))
    }
}

#[test]
fn test_read_num() {
    let correct = String::from("1234;");
    assert_eq!(read_num(&mut correct.chars(), 0), Some((1234, ';')));
}
