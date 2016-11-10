use ui::editor::Editor;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Child, Parent};


pub struct Window {
    editor: Child,
    width: usize,
    height: usize,
}

impl Component for Window {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.editor.comp.resize(width - 2, height - 2);
    }

    fn refresh(&self) -> Vec<Response> {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 200, 200));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        let mut c = self.refresh_children(&mut buffer);
        let mut res = vec![Response::Refresh(0, 0, buffer)];
        res.append(&mut c);
        res
    }
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let mut editor = Editor::new(width, height);
        editor.load_file("LICENSE").unwrap();
        Window {
            editor: Child {
                comp: Box::new(editor),
                x: 1,
                y: 1,
            },
            width: width,
            height: height,
        }
    }
}


impl Parent for Window {
    fn children_mut(&mut self) -> Vec<&mut Child> {
        vec![&mut self.editor]
    }

    fn children(&self) -> Vec<&Child> {
        vec![&self.editor]
    }
}
