/// HQ: Headquarters.
use std::collections::BTreeMap;
use std::path::{self, Path};
use buf::Buffer;
use util::ResultBox;

mod command;

use hq::command::Command;

def_error! {
    NoFileName: "cannot infer filename.",
    InvalidFileName: "cannot decode the filename.",
    Internal: "internal error.",
    NoElement: "no element.",
}

#[derive(Default)]
pub struct Hq {
    buffers: BTreeMap<String, Buffer>,
    commands: BTreeMap<String, Command>
}

impl Hq {
    fn add_command(&mut self,
                   name: &str,
                   args: Vec<String>,
                   func: fn(&mut Hq, &str) -> ResultBox<String>) {
        let res = Command::new(name, args, func);
        self.commands.insert(String::from(name), res);
    }

    pub fn new() -> Hq {
        let mut hq: Hq = Default::default();
        hq.add_command("open-file", vec![String::from("filename")], Hq::open_file);
        hq
    }

    pub fn cmd(&mut self, command: &str, arg: &str) -> ResultBox<String> {
        let t: Vec<_> = self.commands.keys().cloned().collect();
        println!("{:?}", t);
        let func = self.commands.get(command).ok_or(Error::NoElement)?.func;
        func(self, arg)
    }

    fn open_file(&mut self, s: &str) -> ResultBox<String> {
        let file_name = Path::new(s).file_name().ok_or(Error::NoFileName)?
            .to_str().ok_or(Error::InvalidFileName)?;
        let buf = Buffer::from_file(s)?;
        self.buffers.insert(String::from(file_name), buf);
        self.buffers.get_mut(file_name).ok_or(Error::Internal)?.set_cursor(0, 0);
        Ok(String::from(file_name))
    }

    /// Temporary function.
    pub fn buf(&mut self, s: &str) -> Result<&mut Buffer> {
        self.buffers.get_mut(s).ok_or(Error::NoElement)
    }
}
