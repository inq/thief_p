#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Meta(char),
    Esc,
    Home,
    End,
    CR,
    LF,
    Del,
    Up,
    Down,
    Left,
    Right,
}

impl Key {
    /// Convert some events into readable format.
    pub fn normalize(self) -> Key {
        match self {
            Key::Ctrl('j') => Key::LF,
            Key::Ctrl('m') => Key::CR,
            Key::Char('\x7f') => Key::Del,
            etc => etc,
        }
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        String::from(match *self {
                         Key::Char('a') => "C-a",
                         Key::Char('b') => "C-b",
                         _ => "unknown",
                     })
    }
}
