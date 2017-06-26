use term;

#[derive(Debug)]
pub struct ColoredString {
    vec: Vec<term::Char>,
}

const BUFSIZE: usize = 80

impl ColoredString {
    fn new() -> Self {
        Self { vec: Vec::with_capacity(BUFSIZE) },
    }

    /// Discard color and return the string.
    fn as_str(&self) -> &str {
        &self.vec.map(|c| c.chr()).collect()
    }
}
