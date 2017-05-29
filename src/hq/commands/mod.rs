mod command;

use hq::{Arg, Func};
use std::collections::BTreeMap;
pub use self::command::Command;

pub enum Response {
    Func(Func, Vec<String>),
    Require(Arg),
    Message(String),
}

pub struct Commands {
    commands: BTreeMap<String, Command>,
    name: Option<String>,
    args: Vec<String>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            commands: BTreeMap::new(),
            name: None,
            args: vec![],
        }
    }

    pub fn add(&mut self, name: &str, args: Vec<Arg>, func: Func) {
        let res = Command::new(name, args, func);
        self.commands.insert(String::from(name), res);
    }

    /// Receive a function name or argument.
    pub fn query(&mut self, command: &str) -> Response {
        if self.name.is_some() {
            self.args.push(String::from(command));
        } else if self.commands.get(command).is_some() {
            self.name = Some(String::from(command));
        } else {
            return Response::Message(String::from("Not exists the corresponding command."));
        }
        if let Some(ref name) = self.name {
            if let Some(cmd) = self.commands.get(name) {
                if cmd.args_len() == self.args.len() {
                    let mut res = vec![];
                    res.append(&mut self.args);
                    return Response::Func(cmd.func, res);
                } else {
                    return Response::Require(cmd.arg(self.args.len()));
                }
            }
        }
        Response::Message(String::from("Internal error."))
    }
}
