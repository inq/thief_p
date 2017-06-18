use hq;
use term;
use util::ResultBox;
use ui;
use ui::comp::{Component, Parent, View, ViewT};
use ui::hsplit::HSplit;
use ui::command_bar::CommandBar;

def_error! {
    Initialized: "already initialized",
}

#[derive(Default, UiView)]
pub struct Screen {
    view: View,
    hsplit: UiChild,
    command_bar: UiChild,
}

def_child!(UiChild <- HSplit, CommandBar);

impl Component for Screen {
    fn on_resize(&mut self, workspace: &mut hq::Workspace) -> ResultBox<()> {
        self.resize_command_bar(workspace)?;
        let height = self.view.height - self.command_bar().height() - 2;
        self.hsplit.resize(
            workspace,
            1,
            1,
            self.view.width - 2,
            height,
        )
    }

    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response> {
        let rect = term::Rect::new(
            self.view.width,
            self.view.height,
            term::Brush::new(term::Color::new(0, 0, 0), term::Color::new(80, 0, 0)),
        );
        self.refresh_children(rect, workspace)
    }

    /// Propagate to children.
    fn unhandled(
        &mut self,
        workspace: &mut hq::Workspace,
        e: ui::Request,
    ) -> ResultBox<ui::Response> {
        if self.command_bar().focus() {
            self.command_bar.propagate(e, workspace)
        } else {
            self.hsplit.propagate(e, workspace)
        }
    }

    /// Handle keyboard events.
    fn on_key(&mut self, workspace: &mut hq::Workspace, k: term::Key) -> ResultBox<ui::Response> {
        match k {
            term::Key::Ctrl('c') => self.activate_command_bar(workspace),
            _ => Ok(ui::Response::Unhandled),
        }
    }

    /// Send some functions into command bar. Otherwise, into hsplit.
    fn handle(&mut self, workspace: &mut hq::Workspace, e: ui::Request) -> ResultBox<ui::Response> {
        use ui::Request::*;
        match e {
            e @ CommandBar(_) => {
                self.activate_command_bar(workspace)?;
                self.command_bar.propagate(e, workspace)?;
                self.on_resize(workspace)?;
                self.refresh(workspace)
            }
            Resize(width, height) => {
                self.resize(workspace, 0, 0, width, height)?;
                self.refresh(workspace)
            }
            OpenBuffer(_) => {
                if self.view.height > 0 {
                    // After initialize
                    self.command_bar_mut().set_focus(false);
                    self.hsplit.set_focus(true);
                    self.on_resize(workspace)?;
                    self.hsplit.propagate(e, workspace)?;
                    self.refresh(workspace)
                } else {
                    // Before initialize
                    self.hsplit.propagate(e, workspace)?;
                    Ok(ui::Response::None)
                }
            }
            Quit => Ok(ui::Response::Quit),
            _ => Ok(ui::Response::Unhandled),
        }
    }
}

impl Screen {
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
    fn resize_command_bar(&mut self, workspace: &mut hq::Workspace) -> ResultBox<()> {
        self.command_bar.resize(
            workspace,
            0,
            0,
            self.view.width,
            self.view.height,
        )
    }

    /// Activate command bar, and redraw the corresponding area.
    #[inline]
    pub fn activate_command_bar(
        &mut self,
        workspace: &mut hq::Workspace,
    ) -> ResultBox<ui::Response> {
        self.command_bar_mut().set_focus(true);
        self.hsplit.set_focus(false);
        self.resize_command_bar(workspace)?;
        // TODO: Make concise.
        Ok(self.command_bar.refresh(workspace)?.translate(
            self.command_bar.get_view().x,
            self.command_bar.get_view().y,
        ))
    }

    pub fn new() -> Result<Screen> {
        allow_once!();
        Ok(Screen {
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

impl Parent for Screen {
    type Child = UiChild;
    fn children_mut(&mut self) -> Vec<&mut UiChild> {
        vec![&mut self.command_bar, &mut self.hsplit]
            .into_iter()
            .collect()
    }

    fn children(&self) -> Vec<&UiChild> {
        vec![&self.command_bar, &self.hsplit].into_iter().collect()
    }
}

#[test]
fn initialize() {
    assert!(Screen::new().is_ok());
    assert!(Screen::new().is_err());
}
