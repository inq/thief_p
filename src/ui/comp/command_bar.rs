use cmd;
use io::Event;
use ui::res::{Buffer, Brush, Color, Cursor, Char, Line, Response, Refresh, Sequence};
use ui::comp::{Component, View};

pub struct CommandBar {
    pub active: bool,
    data: String,
    view: View,
    background: Brush,
}

impl Default for CommandBar {
    fn default() -> CommandBar {
        CommandBar {
            active: false,
            data: String::with_capacity(80),
            view: Default::default(),
            background: Brush::new(Color::new(220, 220, 220), Color::new(60, 30, 30)),
        }
    }
}

impl Component for CommandBar {
    has_view!();

    /// Force the height to be 1.
    fn on_resize(&mut self) {
        self.view.height = 1;
    }

    /// Handle the keyboard input.
    fn handle(&mut self, e: Event) -> Response {
        match e {
            Event::Ctrl { c: 'm' } => { // Return
                match cmd::query(&self.data) {
                    cmd::Response::Something => {
                        // TODO: do something
                        ()
                    }
                    _ => ()
                }
                Default::default()
            }
            Event::Char { c } => {
                self.data.push(c);
                Response {
                    sequence: vec! [
                        Sequence::Char(Char::new(c, self.background.clone())),
                    ],
                    ..Default::default()
                }
            }
            _ => Default::default()
        }
    }

    fn refresh(&self) -> Response {
        Response {
            refresh: Some(Refresh {
                x: 0,
                y: 0,
                buf: Buffer::blank(&self.background, self.view.width, self.view.height)
            }),
            sequence: vec![
                Sequence::Move(Cursor { x: 0, y: 0 }),
                Sequence::Line(Line::new_from_str(&self.data, &self.background))
            ]
        }
    }
}
