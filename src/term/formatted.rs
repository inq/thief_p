#[derive(Clone, Copy)]
pub enum Style {
    Directory,
    File,
}

struct Entry {
    style: Style,
    token: String,
}

pub struct Formatted {
    entries: Vec<Entry>,
}

impl Formatted {
    pub fn new() -> Formatted {
        Formatted { entries: vec![] }
    }

    pub fn push(mut self, style: Style, token: &str) -> Self {
        self.entries
            .push(Entry {
                      style: style,
                      token: String::from(token),
                  });
        self
    }

    pub fn get(&self, index: usize) -> Option<(Style, char)> {
        let mut acc = 0;
        for entry in &self.entries {
            let next = acc + entry.token.chars().count();
            if acc <= index && next > index {
                return Some((entry.style, entry.token.chars().nth(index - acc).unwrap()));
            }
            acc = next;
        }
        None
    }

    pub fn len(&self) -> usize {
        self.entries.iter().map(|i| i.token.chars().count()).sum()
    }
}
