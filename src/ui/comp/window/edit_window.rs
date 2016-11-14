use io::Event;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Child, Parent};
use super::{Editor};

pub struct EditWindow {
    editor: Child,
    width: usize,
    height: usize,
}

impl Component for EditWindow {
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        self.width = width;
        self.height = height;
        self.editor.x = 0;
        self.editor.y = 0;
        self.editor.comp.resize(width, height);
        (width, height)
    }

    fn refresh(&self) -> Response {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(100, 200, 200));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        self.refresh_children(buffer)
    }

    fn handle(&mut self, e: Event) -> Response {
        match e {
            _ => {
                let res = self.editor.comp.handle(e);
                self.transform(&self.editor, res)
            }
        }
    }
}

impl EditWindow {
    /// Initializer.
    pub fn new() -> Child {
        let editor = Editor::new_with_file("LICENSE").unwrap();
        Child {
            x: usize::max_value(),
            y: usize::max_value(),
            comp: Box::new(EditWindow {
                editor: editor,
                width: usize::max_value(),
                height: usize::max_value(),
            })
        }
    }
}

impl Parent for EditWindow {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        vec![&mut self.editor]
    }

    fn children(&self) -> Vec<&Child> {
        vec![&self.editor]
    }
}
