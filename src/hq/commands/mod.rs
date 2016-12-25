mod command;
use std::collections::BTreeMap;
pub use self::command::{Command, Func, Arg};
use util::ResultBox;
use msg::event;
use hq::Hq;

pub enum Response {
    Func(Func, Vec<String>),
    Message(String)
}

def_error! {
    NoElement: "no element.",
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

    pub fn add(&mut self,
               name: &str,
               args: Vec<Arg>,
               func: Func) {
        let res = Command::new(name, args, func);
        self.commands.insert(String::from(name), res);
    }

    /// Receive a function name or argument.
    pub fn query(&mut self, command: &str) -> Response {
        use msg::event::Event::*;
        use msg::event::CommandBar::*;
        if self.name.is_some() {
            self.args.push(String::from(command));
        } else {
            if self.commands.get(command).is_some() {
                self.name = Some(String::from(command));
            } else {
                return Response::Message(String::from("Not exists the corresponding command."));
            }
        }
        if let Some(ref name) = self.name {
            if let Some(ref cmd) = self.commands.get(name) {
                if cmd.args_len() == self.args.len() {
                    let mut res = vec![];
                    res.append(&mut self.args);
                    return Response::Func(cmd.func, res);
                }
            }
        }
        Response::Message(String::from("Internal error."))
    }
}
