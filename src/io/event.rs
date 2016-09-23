use std::char;

#[derive(Debug)]
pub enum Event {
    Char { c: char },
    Ctrl { c: char },
    Move { x: i8, y: i8 },
    Meta { c: char },
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

    pub fn from_string(s: String) -> (Option<Event>, String) {
        let mut it = s.chars();
        match it.next() {
            Some('\u{1b}') => from_escape(&s),
            Some(c) => (Some(Event::from_char(c)), it.collect()),
            None => (None, it.collect()),
        }
    }
}

fn check_prefix(s: &String, t: &str) -> Option<String> {
    if s.starts_with(&t) {
        Some(s.chars().skip(t.len()).collect())
    } else {
        None
    }
}

fn from_escape(s: &String) -> (Option<Event>, String) {
    if let Some(r) = check_prefix(&s, "\u{1b}[A") {
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
