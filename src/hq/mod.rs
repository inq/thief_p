/// HQ: Headquarters.
use std::collections::BTreeMap;
use std::path;
use buf::Buffer;
use util::ResultBox;

def_error! {
    NoFileName: "cannot infer filename.",
    InvalidFileName: "cannot decode the filename.",
    Internal: "internal error.",
    NoElement: "no element.",
}

#[derive(Default)]
pub struct Hq {
    buffers: BTreeMap<String, Buffer>,
}

impl Hq {
    pub fn open_file<S: AsRef<path::Path> + ?Sized>(&mut self, s: &S) -> ResultBox<String> {
        let file_name = s.as_ref()
            .file_name().ok_or(Error::NoFileName)?
            .to_str().ok_or(Error::InvalidFileName)?;
        let mut buf = Buffer::from_file(s)?;
        self.buffers.insert(String::from(file_name), buf);
        self.buffers.get_mut(file_name).ok_or(Error::Internal)?.set_cursor(0, 0);
        Ok(String::from(file_name))

    }

    /// Temporary function.
    pub fn buf(&mut self, s: &str) -> Result<&mut Buffer> {
        self.buffers.get_mut(s).ok_or(Error::NoElement)
    }
}
