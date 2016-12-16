mod comp;
mod res;
mod hsplit;
mod window;
mod command_bar;

use io::Event;
use hq::Hq;
use util::ResultBox;
use ui::comp::{Parent, View};

pub use ui::comp::Component;
pub use ui::res::*;
use ui::hsplit::HSplit;
use ui::command_bar::CommandBar;

def_error! {
    Initialized: "already initialized",
}

#[derive(Default)]
pub struct Ui {
    view: View,
    hsplit: UiChild,
    command_bar: UiChild,
}

def_child!(UiChild <- HSplit, CommandBar);

impl Component for Ui {
    has_view!();

    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()> {
        self.resize_command_bar(hq)?;
        let height = self.view.height - self.command_bar().height() - 1;
        self.hsplit.resize(hq, 1, 1, self.view.width - 2, height)
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(80, 0, 0));
        let rect = Rect::new(self.view.width, self.view.height, b);
        self.refresh_children(rect, hq)
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        match e {
            Event::Navigate { .. } => {
                self.command_bar.propagate(e, hq)?;
                self.on_resize(hq)?;
                self.refresh(hq)
            }
            Event::Resize { w: width, h: height } => {
                self.resize(hq, 0, 0, width, height)?;
                self.refresh(hq)
            }
            Event::Ctrl { c: 'c' } => self.activate_command_bar(hq),
            Event::Ctrl { c: 'q' } => Ok(Response::quit()),
            Event::OpenBuffer { s: _ } => {
                self.command_bar_mut().active = false;
                self.on_resize(hq)?;
                self.hsplit.propagate(e, hq)?;
                self.refresh(hq)
            }
            _ => {
                if self.command_bar().active {
                    self.command_bar.propagate(e, hq)
                } else {
                    self.hsplit.propagate(e, hq)
                }
            }
        }
    }
}

impl Ui {
    #[inline]
    fn command_bar_mut(&mut self) -> &mut CommandBar {
        if let UiChild::CommandBar(ref mut c) = self.command_bar {
            c
        } else {
            unreachable!()
        }
    }
    #[inline]
    fn command_bar(&self) -> &CommandBar {
        if let UiChild::CommandBar(ref c) = self.command_bar {
            c
        } else {
            unreachable!()
        }
    }

    /// Resize the command bar; the bottom-side of the ui.
    #[inline]
    fn resize_command_bar(&mut self, hq: &mut Hq) -> ResultBox<()> {
        self.command_bar.resize(hq, 0, 0, self.view.width, self.view.height)
    }

    /// Activate command bar, and redrew the corresponding area.
    #[inline]
    pub fn activate_command_bar(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        self.command_bar_mut().active = true;
        self.resize_command_bar(hq)?;
        // TODO: Make concise.
        Ok(self.command_bar
            .refresh(hq)?
            .translate(self.command_bar.get_view().x, self.command_bar.get_view().y))
    }

    pub fn new() -> Result<Ui> {
        allow_once!();
        Ok(Ui {
            hsplit: UiChild::HSplit(HSplit::new(1)),
            command_bar: UiChild::CommandBar(Default::default()),
            ..Default::default()
        })
    }
}

impl Parent for Ui {
    type Child = UiChild;
    fn children_mut(&mut self) -> Vec<&mut UiChild> {
        if self.command_bar().active {
            vec![&mut self.hsplit, &mut self.command_bar]
                .into_iter()
                .collect()
        } else {
            vec![&mut self.hsplit]
                .into_iter()
                .collect()
        }
    }

    fn children(&self) -> Vec<&UiChild> {
        if self.command_bar().active {
            vec![&self.hsplit, &self.command_bar]
                .into_iter()
                .collect()
        } else {
            vec![&self.hsplit]
                .into_iter()
                .collect()
        }
    }
}

#[test]
fn initialize() {
    assert!(Ui::new().is_ok());
    assert!(Ui::new().is_err());
}
