use hq::Hq;
use io::Event;
use util::ResultBox;
use ui::res::{Brush, Color, Cursor, Char, Line, Rect, Response, Refresh, Sequence};
use ui::comp::{Component, View};

#[derive(PartialEq)]
pub enum Status {
    Standby,
    Notify,
    Navigate,
}

pub struct CommandBar {
    pub active: bool,
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
            active: false,
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
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: Rect::new(self.view.width, self.view.height, self.background),
            }),
            sequence: vec![Sequence::Move(Cursor { x: 0, y: 0 }),
                           Sequence::Line(Line::new_from_str(msg, self.background))],
        }
    }

    /// Return the height.
    pub fn height(&self) -> usize {
        if self.active { self.view.height } else { 0 }
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
    fn handle(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        match e {
            Event::Navigate { msg } => {
                // Turn on the navigator
                self.data.clear();
                self.message = String::from(msg);
                self.status = Status::Navigate;
                self.refresh(hq)
            }
            Event::Notify { s } => {
                // Notify from Hq
                Ok(self.notify(&s))
            }
            Event::Ctrl { c: 'm' } => {
                // Return
                Ok(Response {
                    sequence: vec![Sequence::Command(self.data.clone())],
                    ..Default::default()
                })
            }
            Event::Char { c } => {
                match self.status {
                    Status::Standby => {
                        self.data.push(c);
                        Ok(Response {
                            sequence: vec![Sequence::Char(Char::new(c, self.background.clone()))],
                            ..Default::default()
                        })
                    }
                    Status::Notify => {
                        self.status = Status::Standby;
                        self.data.clear();
                        self.data.push(c);
                        self.refresh(hq)
                    }
                    Status::Navigate => {
                        self.data.push(c);
                        Ok(Response {
                            sequence: vec![Sequence::Char(Char::new(c, self.background.clone()))],
                            ..Default::default()
                        })
                    }
                }
            }
            _ => Ok(Default::default()),
        }
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let rect = if self.status == Status::Navigate {
            let mut res = Rect::new(self.view.width, self.view.height, self.background);
            for (i, ref formatted) in hq.fs().unwrap().render().iter().enumerate() {
                res.draw_formatted(formatted, 0, i + 1);
            }
            res
        } else {
            Rect::new(self.view.width, self.view.height, self.background)
        };
        Ok(Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                rect: rect,
            }),
            sequence: vec![Sequence::Move(Cursor { x: 0, y: 0 }),
                           Sequence::Line(Line::new_from_str(&self.data, self.background))],
        })
    }
}
