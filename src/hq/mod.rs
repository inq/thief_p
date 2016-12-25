/// HQ: Headquarters.
use buf::Buffer;
use msg::event;
use util::ResultBox;

mod shortcut;
mod workspace;
mod commands;
mod fs;

use hq::fs::Filesys;
use hq::workspace::Workspace;
use hq::commands::{Commands, Arg};
use hq::shortcut::Shortcut;

pub struct Hq {
    workspace: Workspace,
    commands: Commands,
    shortcut: Shortcut,
}

// TODO: Move these into somewhere.

impl Hq {
    /// Initialize.
    pub fn new() -> ResultBox<Hq> {
        use msg::event::Key;
        let mut commands = Commands::new();
        commands.add("find-file",
                     vec![Arg::Path(String::from("filename"))],
                     Workspace::find_file);
        commands.add("quit", vec![], Workspace::quit);
        let mut shortcut = Shortcut::new();
        shortcut.add("find-file", vec![Key::Ctrl('x'), Key::Ctrl('f')]);
        shortcut.add("quit", vec![Key::Ctrl('x'), Key::Ctrl('c')]);
        Ok(Hq {
            workspace: Workspace::new()?,
            commands: commands,
            shortcut: shortcut,
        })
    }

    /// Consume event before UI.
    pub fn preprocess(&mut self, e: event::Event) -> event::Event {
        use msg::event::Event::{CommandBar, Keyboard};
        use msg::event::CommandBar::Shortcut;
        use self::shortcut::Response;
        match e {
            Keyboard(k) => {
                match self.shortcut.key(k) {
                    Response::More(s) => CommandBar(Shortcut(s)),
                    Response::Some(s) => self.call(&s).unwrap(),
                    _ => e,
                }
            }
            _ => e,
        }
    }

    pub fn call(&mut self, command: &str) -> Option<event::Event> {
        use msg::event::Event::*;
        use msg::event::CommandBar::*;
        use self::commands::Response::*;
        use self::commands::Arg;
        match self.commands.query(command) {
            Func(func, args) => func(&mut self.workspace, args).ok(),
            Require(Arg::Path(_)) => Some(CommandBar(Navigate(String::from(".")))),
            Require(Arg::String(_)) => unimplemented!(),
            Message(m) => Some(CommandBar(Notify(m))),
        }
    }

    pub fn fs(&mut self) -> ResultBox<&mut Filesys> {
        Ok(self.workspace.fs())
    }

    // pub fn run(&mut self, command: &str, arg: &str) -> ResultBox<String> {
    // let func = self.commands.query(command)?;
    // func(self, &vec![String::from(arg)])
    // }

    /// Temporary function.
    pub fn buf(&mut self, s: &str) -> ResultBox<&mut Buffer> {
        self.workspace.get_mut(s)
    }
}
