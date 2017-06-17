use hq;
use term;
use ui;
use util::ResultBox;
use ui::comp::{Component, ViewT};

#[derive(PartialEq)]
pub enum Status {
    Standby,
    Notify,
    Navigate,
    Shortcut,
}

#[derive(UiView)]
pub struct CommandBar {
    view: ViewT,
    status: Status,
    data: String,
    message: String,
    background: term::Brush,
}

impl Default for CommandBar {
    fn default() -> CommandBar {
        CommandBar {
            status: Status::Standby,
            data: String::with_capacity(80),
            message: String::with_capacity(80),
            view: Default::default(),
            background: term::Brush::new(
                term::Color::new(220, 220, 220),
                term::Color::new(60, 30, 30),
            ),
        }
    }
}

impl CommandBar {
    /// Notify a given message.
    fn notify(&mut self, msg: &str) -> ui::Response {
        self.status = Status::Notify;
        let mut rect = term::Rect::new(self.view.width, self.view.height, self.background);
        rect.draw_str(msg, 0, 0);
        ui::Response::Term {
            refresh: Some(term::Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            cursor: None,
        }
    }

    /// Return the height.
    pub fn height(&self) -> usize {
        if self.focus() { self.view.height } else { 1 }
    }

    fn handle_command_bar(
        &mut self,
        c: ui::CommandBar,
        workspace: &mut hq::Workspace,
    ) -> ResultBox<ui::Response> {
        match c {
            ui::CommandBar::Navigate(msg) => {
                // Turn on the navigator
                self.data.clear();
                self.message = String::from(msg);
                self.status = Status::Navigate;
                self.refresh(workspace)
            }
            ui::CommandBar::Shortcut(s) => {
                self.message = String::from(s.clone());
                self.status = Status::Shortcut;
                self.refresh(workspace)
            }
            ui::CommandBar::Notify(s) => Ok(self.notify(&s)),
        }
    }
}

impl Component for CommandBar {
    /// Force the height.
    fn on_resize(&mut self, _: &mut hq::Workspace) -> ResultBox<()> {
        let height_parent = self.view.height;
        self.view.height = if self.status == Status::Navigate {
            height_parent / 3
        } else {
            1
        };
        self.view.y = height_parent - self.view.height;
        Ok(())
    }

    /// Handle the keyboard input.
    fn on_key(&mut self, workspace: &mut hq::Workspace, k: term::Key) -> ResultBox<ui::Response> {
        match k {
            term::Key::CR => Ok(ui::Response::Command(self.data.clone())),
            term::Key::Char(c) => {
                use self::Status::*;
                match self.status {
                    Standby | Navigate => {
                        // TODO: Must consider unicode.
                        let prev = self.data.len();
                        self.data.push(c);
                        Ok(ui::Response::Term {
                            refresh: Some(term::Refresh {
                                x: prev,
                                y: 0,
                                rect: term::Rect::new_from_char(
                                    term::Char::new(c, self.background),
                                ),
                            }),
                            cursor: Some((self.data.len(), 0)),
                        })
                    }
                    Notify => {
                        self.status = Status::Standby;
                        self.data.clear();
                        self.data.push(c);
                        self.refresh(workspace)
                    }
                    Shortcut => unreachable!(),
                }
            }
            _ => Ok(ui::Response::None),
        }
    }

    /// Handle events.
    fn handle(&mut self, workspace: &mut hq::Workspace, e: ui::Request) -> ResultBox<ui::Response> {
        match e {
            ui::Request::CommandBar(c) => self.handle_command_bar(c, workspace),
            _ => Ok(ui::Response::None),
        }
    }

    /// Refresh the command bar.
    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response> {
        let mut rect = if self.status == Status::Navigate {
            let mut res = term::Rect::new(self.view.width, self.view.height, self.background);
            for (i, formatted) in workspace.fs().render().iter().enumerate() {
                res.draw_formatted(formatted, 0, i + 1);
            }
            res
        } else {
            term::Rect::new(self.view.width, self.view.height, self.background)
        };
        rect.draw_str(&self.message, 0, 0);
        Ok(ui::Response::Term {
            refresh: Some(term::Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            cursor: Some((0, 0)),
        })
    }
}
