use msg::event;
use util::ResultBox;
use ui::{self, Component};
use term;

use hq::enums::Arg;
use hq::commands::Commands;
use hq::shortcut::Shortcut;
use hq::workspace::Workspace;

pub struct Handler {
    screen: ui::Screen,
    workspace: Workspace,
    commands: Commands,
    shortcut: Shortcut,
}

impl Handler {
    /// Initialize.
    pub fn new(screen: ui::Screen) -> ResultBox<Handler> {
        use msg::event::Key;
        let mut commands = Commands::new();
        commands.add("find-file",
                     vec![Arg::Path(String::from("filename"))],
                     Workspace::find_file);
        commands.add("quit", vec![], Workspace::quit);
        let mut shortcut = Shortcut::new();
        shortcut.add("find-file", vec![Key::Ctrl('x'), Key::Ctrl('f')]);
        shortcut.add("quit", vec![Key::Ctrl('x'), Key::Ctrl('c')]);
        Ok(Handler {
               screen,
               workspace: Workspace::new()?,
               commands: commands,
               shortcut: shortcut,
           })
    }

    /// Consume event from Io.
    pub fn request(&mut self, e: event::Event) -> ResultBox<term::Response> {
        use msg::event::Event::{CommandBar, Keyboard};
        use msg::event::CommandBar::Shortcut;
        use hq::shortcut::Response;
        let e = if let Keyboard(k) = e {
            match self.shortcut.key(k) {
                Response::More(s) => CommandBar(Shortcut(s)),
                Response::Some(s) => self.call(&s).unwrap(),
                _ => e,
            }
        } else {
            e
        };
        self.handle_event(e)
    }

    fn handle_event(&mut self, e: event::Event) -> ResultBox<term::Response> {
        self.screen.propagate(e, &mut self.workspace)
    }

    /// Run a given command.
    pub fn call(&mut self, command: &str) -> Option<event::Event> {
        use msg::event::Event::*;
        use msg::event::CommandBar::*;
        use hq::commands::Response::*;
        match self.commands.query(command) {
            Func(func, args) => func(&mut self.workspace, args).ok(),
            Require(Arg::Path(_)) => Some(CommandBar(Navigate(String::from(".")))),
            Message(m) => Some(CommandBar(Notify(m))),
        }
    }
}
