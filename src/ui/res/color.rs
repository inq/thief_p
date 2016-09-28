#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Brush {
    fg: Color,
    bg: Color,
}

impl Brush {
    pub fn new(fg: Color, bg: Color) -> Brush {
        Brush { fg: fg, bg: bg }
    }

    pub fn change(from: &Option<Brush>, to: &Option<Brush>) -> String {
        match (from, to) {
            (&Some(ref f), &Some(ref t)) => {
                let mut res = String::from("");
                if f.fg != t.fg {
                    res += &format!("\u{1b}[38;2;{};{};{}m", t.fg.r, t.fg.g, t.fg.b);
                }
                if f.bg != t.bg {
                    res += &format!("\u{1b}[48;2;{};{};{}m", t.bg.r, t.bg.g, t.bg.b);
                }
                res
            },
            (&None, &Some(ref t)) =>
                format!("\u{1b}[38;2;{};{};{}m\u{1b}[48;2;{};{};{}m",
                        t.fg.r, t.fg.g, t.fg.b, t.bg.r, t.bg.g, t.bg.b),
            (&Some(ref f), &None) => String::from("\u{1b}[0m"),
            (&None, &None) => String::from(""),
        }
    }

    pub fn invert(&self) -> Brush {
        Brush {
            fg: self.bg.clone(),
            bg: self.fg.clone(),
        }
    }
}
