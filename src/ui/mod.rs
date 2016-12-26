mod comp;
mod res;
mod hsplit;
mod theme;
mod window;
mod command_bar;

use msg::event;
use hq::Hq;
use util::ResultBox;
use ui::comp::{Parent, View};

pub use ui::comp::Component;
pub use ui::res::*;
use ui::theme::Theme;
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
        let height = self.view.height - self.command_bar().height() - 2;
        self.hsplit.resize(hq, 1, 1, self.view.width - 2, height)
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let rect = Rect::new(self.view.width,
                             self.view.height,
                             Brush::new(Color::new(0, 0, 0), Color::new(80, 0, 0)));
        self.refresh_children(rect, hq)
    }

    /// Propagate to children.
    fn unhandled(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        if self.command_bar().focus() {
            self.command_bar.propagate(e, hq)
        } else {
            self.hsplit.propagate(e, hq)
        }
    }

    /// Handle keyboard events.
    fn on_key(&mut self, hq: &mut Hq, k: event::Key) -> ResultBox<Response> {
        use msg::event::Key::*;
        match k {
            Ctrl('c') => self.activate_command_bar(hq),
            _ => Ok(Response::Unhandled),
        }
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, hq: &mut Hq, e: event::Event) -> ResultBox<Response> {
        use msg::event::Event::*;
        match e {
            e @ CommandBar(_) => {
                self.activate_command_bar(hq)?;
                self.command_bar.propagate(e, hq)?;
                self.on_resize(hq)?;
                self.refresh(hq)
            }
            Resize(width, height) => {
                self.resize(hq, 0, 0, width, height)?;
                self.refresh(hq)
            }
            OpenBuffer(_) => {
                if self.view.height > 0 {
                    // After initialize
                    self.command_bar_mut().set_focus(false);
                    self.hsplit.set_focus(true);
                    self.on_resize(hq)?;
                    self.hsplit.propagate(e, hq)?;
                    self.refresh(hq)
                } else {
                    // Before initialize
                    self.hsplit.propagate(e, hq)?;
                    Ok(Default::default())
                }
            }
            Quit => Ok(Response::Quit),
            _ => Ok(Response::Unhandled),
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

    /// Activate command bar, and redraw the corresponding area.
    #[inline]
    pub fn activate_command_bar(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        self.command_bar_mut().set_focus(true);
        self.hsplit.set_focus(false);
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
            command_bar: {
                let mut res: CommandBar = Default::default();
                res.set_focus(false);
                UiChild::CommandBar(res)
            },
            ..Default::default()
        })
    }
}

impl Parent for Ui {
    type Child = UiChild;
    fn children_mut(&mut self) -> Vec<&mut UiChild> {
        vec![&mut self.command_bar, &mut self.hsplit]
            .into_iter()
            .collect()
    }

    fn children(&self) -> Vec<&UiChild> {
        vec![&self.command_bar, &self.hsplit]
            .into_iter()
            .collect()
    }
}

#[test]
fn initialize() {
    assert!(Ui::new().is_ok());
    assert!(Ui::new().is_err());
}
