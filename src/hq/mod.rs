/// HQ: Headquarters.
use std::collections::BTreeMap;
use std::path::Path;
use buf::Buffer;
use msg::event;
use util::ResultBox;

mod shortcut;
mod command;
mod fs;

use hq::fs::Filesys;
use hq::command::Command;
use hq::shortcut::Shortcut;

def_error! {
    NoFileName: "cannot infer filename.",
    InvalidFileName: "cannot decode the filename.",
    Internal: "internal error.",
    NoElement: "no element.",
}

pub struct Hq {
    buffers: BTreeMap<String, Buffer>,
    commands: BTreeMap<String, Command>,
    current: Vec<String>,
    shortcut: Shortcut,
    fs: Filesys,
}

impl Hq {
    fn add_command(&mut self,
                   name: &str,
                   args: Vec<String>,
                   func: fn(&mut Hq, &str) -> ResultBox<String>) {
        let res = Command::new(name, args, func);
        self.commands.insert(String::from(name), res);
    }

    /// Initialize.
    pub fn new() -> ResultBox<Hq> {
        use msg::event::Key;
        let mut hq = Hq {
            buffers: Default::default(),
            commands: Default::default(),
            current: Default::default(),
            shortcut: Shortcut::new(),
            fs: Filesys::new()?,
        };
        hq.add_command("find-file", vec![String::from("filename")], Hq::open_file);
        hq.shortcut.add("find-file", vec![Key::Ctrl('x'), Key::Ctrl('f')]);
        hq.shortcut.add("quit", vec![Key::Ctrl('x'), Key::Ctrl('c')]);
        hq.buffers.insert(String::from("<empty>"), Default::default());
        Ok(hq)
    }

    /// Consume event before UI.
    pub fn preprocess(&mut self, e: event::Event) -> event::Event {
        use msg::event::Event::{CommandBar, Keyboard};
        use msg::event::CommandBar::{Shortcut};
        use self::shortcut::Response;
        match e {
            Keyboard(k) => {
                match self.shortcut.key(k) {
                    Response::More(s) => CommandBar(Shortcut(s)),
                    Response::Some(s) => {
                        // Run the command
                        self.call(&s).unwrap()
                    },
                    _ => e,
                }
            }
            _ => e,
        }
    }

    pub fn fs(&mut self) -> Result<&mut Filesys> {
        Ok(&mut self.fs)
    }

    /// Receive a function name or argument.
    pub fn call(&mut self, command: &str) -> Option<event::Event> {
        use msg::event::Event::*;
        use msg::event::CommandBar::*;
        let cmd = if self.current.len() == 0 {
            // function name
            if let Some(_) = self.commands.get(command) {
                self.current.push(String::from(command));
                CommandBar(Navigate(String::from("find-file: ")))
            } else {
                CommandBar(Notify(String::from("Not exists the corresponding command.")))
            }
        } else {
            // argument
            if let Some(_) = self.commands.get(&self.current[0]) {
                let funcname = self.current[0].clone();
                if let Ok(bufname) = self.run(&funcname, command) {
                    OpenBuffer(String::from(bufname))
                } else {
                    CommandBar(Notify(String::from("Cannot open the file.")))
                }
            } else {
                self.current.clear();
                CommandBar(Notify(String::from("Internal error.")))
            }
        };
        Some(cmd)
    }

    pub fn run(&mut self, command: &str, arg: &str) -> ResultBox<String> {
        let func = self.commands.get(command).ok_or(Error::NoElement)?.func;
        func(self, arg)
    }

    fn open_file(&mut self, s: &str) -> ResultBox<String> {
        let file_name = Path::new(s).file_name()
            .ok_or(Error::NoFileName)?
            .to_str()
            .ok_or(Error::InvalidFileName)?;
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
