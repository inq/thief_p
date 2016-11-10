use ui::editor::Editor;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Child, Parent};

pub struct EditWindow {
    editor: Child,
    width: usize,
    height: usize,
}

impl Component for EditWindow {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.editor.comp.resize(width - 3, height);
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(100, 200, 200));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut c = self.refresh_children(&mut buffer);
        let mut res = vec![Response::Refresh(0, 0, buffer)];
        res.append(&mut c);
        res
    }
}

impl EditWindow {
    pub fn new(width: usize, height: usize) -> EditWindow {
        let mut editor = Editor::new(width, height);
        editor.load_file("LICENSE").unwrap();
        EditWindow {
            editor: Child {
                comp: Box::new(editor),
                x: 3,
                y: 0,
            },
            width: width,
            height: height,
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
