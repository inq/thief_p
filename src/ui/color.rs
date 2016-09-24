#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b}
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

    pub fn change(&self, to: &Brush) {
        if self.fg != to.fg {
            print!("\u{1b}[38;2;{};{};{}m", to.fg.r, to.fg.g, to.fg.b);
        }
        if self.bg != to.bg {
            print!("\u{1b}[48;2;{};{};{}m", to.bg.r, to.bg.g, to.bg.b);
        }
    }

    pub fn invert(&self) -> Brush {
        Brush { fg: self.bg.clone(), bg: self.fg.clone() }
    }
}
