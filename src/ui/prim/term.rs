pub fn movexy(buf: &mut String, x: usize, y: usize) {
    buf.push_str(&format!("\u{1b}[{};{}f", y + 1, x + 1));
}
