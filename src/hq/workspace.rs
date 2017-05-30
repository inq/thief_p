use ui;
use std::collections::BTreeMap;
use std::path::Path;
use hq::fs::Filesys;
use buf::Buffer;
use util::ResultBox;

def_error! {
    NoElement: "no element.",
    NoFileName: "cannot infer filename.",
    Internal: "internal error.",
    InvalidFileName: "cannot decode the filename.",
}

pub struct Workspace {
    buffers: BTreeMap<String, Buffer>,
    fs: Filesys,
}

impl Workspace {
    pub fn new() -> ResultBox<Workspace> {
        let mut res = Workspace {
            buffers: BTreeMap::new(),
            fs: Filesys::new()?,
        };
        // TODO: Refactor me!
        res.buffers
            .insert(String::from("<empty>"), Default::default());
        Ok(res)
    }

    pub fn fs(&mut self) -> &mut Filesys {
        &mut self.fs
    }

    pub fn buf(&mut self, s: &str) -> ResultBox<&mut Buffer> {
        self.buffers
            .get_mut(s)
            .ok_or_else(|| From::from(Error::NoElement))
    }

    pub fn find_file(&mut self, args: Vec<String>) -> ResultBox<ui::Request> {
        let s = &args[0];
        let file_name = Path::new(s)
            .file_name()
            .ok_or(Error::NoFileName)?
            .to_str()
            .ok_or(Error::InvalidFileName)?;
        let buf = Buffer::from_file(&s)?;
        self.buffers.insert(String::from(file_name), buf);
        self.buffers
            .get_mut(file_name)
            .ok_or(Error::Internal)?
            .set_cursor(0, 0);
        Ok(ui::Request::OpenBuffer(String::from(file_name)))
    }

    pub fn quit(&mut self, _: Vec<String>) -> ResultBox<ui::Request> {
        Ok(ui::Request::Quit)
    }
}
