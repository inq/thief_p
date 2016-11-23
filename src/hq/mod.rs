/// HQ: Headquarters.
use std::path;
use buf::Buffer;
use util::ResultBox;

def_error! {
    Internal: "internal error.",
    NoElement: "no element.",
}

#[derive(Default)]
pub struct Hq {
    buffers: Vec<Buffer>,
}

impl Hq {
    pub fn open_file<S: AsRef<path::Path> + ?Sized>(&mut self, s: &S) -> ResultBox<&mut Buffer> {
        self.buffers.push(Buffer::from_file(s)?);
        if let Some(b) = self.buffers.last_mut() {
            b.set_cursor(0, 0);
            Ok(b)
        } else {
            Err(From::from(Error::Internal))
        }
    }

    /// Temporary function.
    pub fn buf(&mut self) -> Result<&mut Buffer> {
        self.buffers.last_mut().ok_or(Error::NoElement)
    }
}
