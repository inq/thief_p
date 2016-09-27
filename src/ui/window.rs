use ui::editor::Editor;
use ui::res::{Response, Buffer, Brush, Color};
use ui::comp::{Component, Child, Parent};

pub struct Window {
    editor: Child<Editor>,
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
        let buffer = Buffer::blank(&b, self.width, self.height);
        self.refresh_children(buffer)
    }
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        Window {
            editor: Child {
                comp: Editor::new(width, height),
                x: 1,
                y: 1,
            },
            width: width,
            height: height,
        }
    }
}


impl Parent<Editor> for Window {
    fn children_mut(&mut self) -> Vec<&mut Child<Editor>> {
        vec![&mut self.editor]
    }

    fn children(&self) -> Vec<&Child<Editor>> {
        vec![&self.editor]
    }
}
