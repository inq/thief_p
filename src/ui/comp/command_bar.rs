use hq::Hq;
use io::Event;
use ui::res::{Buffer, Brush, Color, Cursor, Char, Line, Response, Refresh, Sequence};
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
    view: View,
    background: Brush,
}

impl Default for CommandBar {
    fn default() -> CommandBar {
        CommandBar {
            status: Status::Standby,
            active: false,
            data: String::with_capacity(80),
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
                buf: Buffer::blank(&self.background, self.view.width, self.view.height),
            }),
            sequence: vec![Sequence::Move(Cursor { x: 0, y: 0 }),
                           Sequence::Line(Line::new_from_str(msg, &self.background))],
        }
    }

    /// Return the height.
    pub fn height(&self) -> usize {
        self.view.height
    }
}

impl Component for CommandBar {
    has_view!();

    /// Force the height.
    fn on_resize(&mut self) {
        let height_parent = self.view.height;
        self.view.height = if self.status == Status::Navigate {
            height_parent / 3
        } else {
            1
        };
        self.view.y = height_parent - self.view.height;
    }

    /// Handle the keyboard input.
    fn handle(&mut self, e: Event, hq: &mut Hq) -> Response {
        match e {
            Event::Navigate { msg } => {
                // Turn on the navigator
                self.status = Status::Navigate;
                self.refresh(hq)
            }
            Event::Notify { s } => {
                // Notify from Hq
                self.notify(&s)
            }
            Event::Ctrl { c: 'm' } => {
                // Return
                Response {
                    sequence: vec![Sequence::Command(self.data.clone())],
                    ..Default::default()
                }
            }
            Event::Char { c } => {
                match self.status {
                    Status::Standby => {
                        self.data.push(c);
                        Response {
                            sequence: vec![Sequence::Char(Char::new(c, self.background.clone()))],
                            ..Default::default()
                        }
                    }
                    Status::Notify => {
                        self.status = Status::Standby;
                        self.data.clear();
                        self.data.push(c);
                        self.refresh(hq)
                    }
                    Status::Navigate => self.refresh(hq),
                }
            }
            _ => Default::default(),
        }
    }

    fn refresh(&self, hq: &mut Hq) -> Response {
        let buf = if self.status == Status::Navigate {
            let mut res = Buffer::blank(&self.background, self.view.width, self.view.height);
            for (i, ref formatted) in hq.fs().unwrap().render().iter().enumerate() {
                res.draw_formatted(formatted, 2, i);
            }
            res
        } else {
            Buffer::blank(&self.background, self.view.width, self.view.height)
        };
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                buf: buf,
            }),
            sequence: vec![Sequence::Move(Cursor { x: 0, y: 0 }),
                           Sequence::Line(Line::new_from_str(&self.data, &self.background))],
        }
    }
}
