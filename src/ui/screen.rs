use std::error;
use ui::buffer::Buffer;
use ui::color::{Brush, Color};
use ui::term;

pub struct Screen {}

impl Screen {
    pub fn new() -> Screen {
        Screen {}
    }

    pub fn refresh(&self, mut buf: &mut String) -> Result<(), Box<error::Error>> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 250, 250));

        let (w, h) = try!(term::get_size());
        let buffer = Buffer::bordered(&b, &b.invert(), w, h);
        buffer.print(&mut buf, &b.invert());
        Ok(())
    }
}
