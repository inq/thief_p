use hq::Hq;
use io::Event;
use util::ResultBox;
use ui::res::{Response, Rect, Brush, Color};
use ui::comp::{Component, Parent, View};
use super::Editor;

#[derive(Default)]
pub struct Edit {
    view: View,
    editor: Editor,
}

impl Component for Edit {
    has_view!();

    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()> {
        self.editor.resize(hq, 0, 0, self.view.width, self.view.height)
    }

    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(100, 200, 200));
        let rect = Rect::new(self.view.width, self.view.height, b);
        self.refresh_children(rect, hq)
    }

    fn handle(&mut self, e: Event, hq: &mut Hq) -> ResultBox<Response> {
        match e {
            _ => self.editor.propagate(e, hq),
        }
    }
}

impl Edit {
    /// Initializer.
    pub fn new() -> Edit {
        Edit { editor: Editor::new(), ..Default::default() }
    }
}

impl Parent for Edit {
    type Child = Editor;
    fn children_mut(&mut self) -> Vec<&mut Editor> {
        vec![&mut self.editor]
    }

    fn children(&self) -> Vec<&Editor> {
        vec![&self.editor]
    }
}
