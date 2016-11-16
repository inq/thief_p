use io::Event;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Parent, View};
use super::{Editor};

#[derive(Default)]
pub struct Edit {
    view: View,
    editor: Editor,
}

impl Component for Edit {
    fn get_view(&self) -> &View {
        &self.view
    }

    fn resize(&mut self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        self.view.x = x;
        self.view.y = y;
        self.view.width = width;
        self.view.height = height;
        self.editor.resize(0, 0, width, height);
        (width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(100, 200, 200));
        let buffer = Buffer::blank(&b, self.view.width, self.view.height);

        self.refresh_children(buffer)
    }

    fn handle(&mut self, e: Event) -> Response {
        match e {
            _ => {
                let res = self.editor.handle(e);
                self.transform(res)
            }
        }
    }
}

impl Edit {
    /// Initializer.
    pub fn new() -> Edit {
        Edit {
            editor: Editor::new_with_file("LICENSE").unwrap(),
            ..Default::default()
        }
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