use hq;
use util::ResultBox;
use term;
use ui;
use ui::comp::{ViewT, Parent, Component};
use ui::editor::Editor;

#[derive(Default, UiView)]
pub struct HSplit {
    view: ViewT,
    editors: Vec<Editor>,
    focused: usize,
}

impl Component for HSplit {
    /// Resize each child editors.
    fn on_resize(&mut self, workspace: &mut hq::Workspace) -> ResultBox<()> {
        let editors = self.editors.len();
        let borders = editors + 1;
        let mut offset = 1;
        for (i, &mut ref mut child) in self.editors.iter_mut().enumerate() {
            let w = (self.view.width - borders + i) / editors;
            child.resize(workspace, offset, 1, w, self.view.height - 2)?;
            offset += w + 1;
        }
        Ok(())
    }

    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response> {
        let rect = term::Rect::new(
            self.view.width,
            self.view.height,
            term::Brush::new(term::Color::new(0, 0, 0), term::Color::new(200, 250, 250)),
        );
        self.refresh_children(rect, workspace)
    }

    /// Propagate if the event is not handled.
    fn unhandled(
        &mut self,
        workspace: &mut hq::Workspace,
        e: ui::Request,
    ) -> ResultBox<ui::Response> {
        self.editors[self.focused].propagate(e, workspace)
    }

    /// Handle the keyboard event.
    fn on_key(&mut self, workspace: &mut hq::Workspace, k: term::Key) -> ResultBox<ui::Response> {
        match k {
            term::Key::Ctrl('d') => {
                self.toggle_split(workspace)?;
                self.refresh(workspace)
            }
            _ => Ok(ui::Response::Unhandled),
        }
    }
}

impl HSplit {
    fn toggle_split(&mut self, workspace: &mut hq::Workspace) -> ResultBox<()> {
        let ws = self.editors.len() % 3 + 1;
        self.set_children(ws);
        let x = self.view.x;
        let y = self.view.y;
        let w = self.view.width;
        let h = self.view.height;
        self.resize(workspace, x, y, w, h)
    }

    pub fn set_children(&mut self, children: usize) {
        // TODO: Must be implemented
        self.focused = 0;
        if children <= self.editors.len() {
            self.editors.truncate(children)
        } else {
            for _ in 0..(children - self.editors.len()) {
                self.editors.push(Editor::new())
            }
        }
    }

    pub fn new(editors: usize) -> HSplit {
        let mut res: HSplit = Default::default();
        res.set_children(editors);
        res
    }
}

impl Parent for HSplit {
    type Child = Editor;
    fn children_mut(&mut self) -> Vec<&mut Editor> {
        self.editors.iter_mut().collect()
    }

    fn children(&self) -> Vec<&Editor> {
        self.editors.iter().collect()
    }
}
