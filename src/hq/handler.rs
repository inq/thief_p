use util::ResultBox;
use ui::{self, Component};
use term;

use hq;
use hq::enums::Arg;
use hq::commands::{self, Commands};
use hq::shortcut::Shortcut;
use hq::workspace::Workspace;

/// `io::Handler` -- `Request` -->
///              <-- `Response`--  `hq::Handler`
pub struct Handler {
    screen: ui::Screen,
    workspace: Workspace,
    commands: Commands,
    shortcut: Shortcut,
}

impl Handler {
    /// Initialize.
    pub fn new(screen: ui::Screen) -> ResultBox<Handler> {
        let mut commands = Commands::new();
        let mut shortcut = Shortcut::new();
        commands.add(
            "find-file",
            vec![Arg::Path(String::from("filename"))],
            Workspace::find_file,
        );
        commands.add("quit", vec![], Workspace::quit);
        shortcut.add(
            "find-file",
            vec![term::Key::Ctrl('x'), term::Key::Ctrl('f')],
        );
        shortcut.add("quit", vec![term::Key::Ctrl('x'), term::Key::Ctrl('c')]);
        Ok(Handler {
            screen,
            workspace: Workspace::new()?,
            commands: commands,
            shortcut: shortcut,
        })
    }

    /// Consume event from Io.
    pub fn request(&mut self, e: hq::Request) -> ResultBox<hq::Response> {
        use hq::shortcut;
        let e = if let hq::Request::Keyboard(k) = e {
            match self.shortcut.key(k) {
                shortcut::Response::More(s) => ui::Request::CommandBar(ui::CommandBar::Shortcut(s)),
                shortcut::Response::Some(s) => self.call(&s).unwrap(),
                _ => e.into_ui(),
            }
        } else {
            e.into_ui()
        };
        self.handle_event(e)
    }

    fn handle_event(&mut self, e: ui::Request) -> ResultBox<hq::Response> {
        match self.screen.propagate(e, &mut self.workspace)? {
            ui::Response::Quit => Ok(hq::Response::Quit),
            ui::Response::Term { refresh, cursor } => Ok(hq::Response::Term { refresh, cursor }),
            ui::Response::None => Ok(hq::Response::None),
            ui::Response::Command(s) => {
                let req = self.call(&s).unwrap();
                self.handle_event(req)
            }
            e => panic!("{:?}", e),
        }
    }

    /// Run a given command.
    pub fn call(&mut self, command: &str) -> Option<ui::Request> {
        match self.commands.query(command) {
            commands::Response::Func(func, args) => func(&mut self.workspace, args).ok(),
            commands::Response::Require(Arg::Path(_)) => {
                Some(ui::Request::CommandBar(
                    ui::CommandBar::Navigate(String::from(".")),
                ))
            }
            commands::Response::Message(m) => {
                Some(ui::Request::CommandBar(ui::CommandBar::Notify(m)))
            }
        }
    }
}
