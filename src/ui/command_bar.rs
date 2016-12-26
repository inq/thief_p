use msg::event;
use hq::Hq;
use util::ResultBox;
use ui::res::{Brush, Color, Cursor, Char, Line, Rect, Response, Refresh, Sequence};
use ui::comp::{Component, View};

#[derive(PartialEq)]
pub enum Status {
    Standby,
    Notify,
    Navigate,
    Shortcut,
}

pub struct CommandBar {
    status: Status,
    data: String,
    message: String,
    view: View,
    background: Brush,
}

impl Default for CommandBar {
    fn default() -> CommandBar {
        CommandBar {
            status: Status::Standby,
            data: String::with_capacity(80),
            message: String::with_capacity(80),
            view: Default::default(),
            background: Brush::new(Color::new(220, 220, 220), Color::new(60, 30, 30)),
        }
    }
}

impl CommandBar {
    /// Notify a given message.
    fn notify(&mut self, msg: &str) -> Response {
        self.status = Status::Notify;
        let mut rect = Rect::new(self.view.width, self.view.height, self.background);
        rect.draw_str(msg, 0, 0);
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            cursor: None,
            sequence: vec![],
        }
    }

    /// Return the height.
    pub fn height(&self) -> usize {
        if self.focus() { self.view.height } else { 1 }
    }

    fn handle_command_bar(&mut self, c: event::CommandBar, hq: &mut Hq) -> ResultBox<Response> {
        use msg::event::CommandBar::*;
        match c {
            Navigate(msg) => {
                // Turn on the navigator
                self.data.clear();
                self.message = String::from(msg);
                self.status = Status::Navigate;
                self.refresh(hq)
            }
            Shortcut(s) => {
                self.message = String::from(s.clone());
                self.status = Status::Shortcut;
                self.refresh(hq)
            }
            Notify(s) => Ok(self.notify(&s)),
        }
    }
}

impl Component for CommandBar {
    has_view!();

    /// Force the height.
    fn on_resize(&mut self, _: &mut Hq) -> ResultBox<()> {
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
    fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        use msg::event::Key;
        match k {
            Key::CR => {
                Ok(Response {
                    sequence: vec![Sequence::Command(self.data.clone())],
                    ..Default::default()
                })
            }
            Key::Char(c) => {
                use self::Status::*;
                match self.status {
                    Standby | Navigate => {
                        // TODO: Must consider unicode.
                        let prev = self.data.len();
                        self.data.push(c);
                        Ok(Response {
                            refresh: Some(Refresh {
                                x: prev,
                                y: 0,
                                rect: Rect::new_from_char(Char::new(c, self.background)),
                            }),
                            cursor: Some(Cursor { x: self.data.len(), y: 0 }),
                            ..Default::default()
                        })
                    }
                    Notify => {
                        self.status = Status::Standby;
                        self.data.clear();
                        self.data.push(c);
                        self.refresh(hq)
                    }
                    Shortcut => unreachable!(),
                }
            }
            _ => Ok(Default::default()),
        }
    }

    /// Handle events.
    fn handle(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        use msg::event::Event::*;
        match e {
            CommandBar(c) => self.handle_command_bar(c, hq),
            _ => Ok(Default::default()),
        }
    }

    /// Refresh the command bar.
    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let mut rect = if self.status == Status::Navigate {
            let mut res = Rect::new(self.view.width, self.view.height, self.background);
            for (i, formatted) in hq.fs().unwrap().render().iter().enumerate() {
                res.draw_formatted(formatted, 0, i + 1);
            }
            res
        } else {
            Rect::new(self.view.width, self.view.height, self.background)
        };
        rect.draw_str(&self.message, 0, 0);
        Ok(Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            sequence: vec![],
            cursor: Some(Cursor{ x: 0, y: 0 }),
        })
    }
}
