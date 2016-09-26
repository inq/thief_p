use ui::editor::Editor;
use ui::prim::{Buffer, Brush, Color};
use ui::comp::{Component, Child};

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
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        Window {
            editor: Child { comp: Editor::new(width, height), x: 1, y: 1 },
            width: width,
            height: height
        }
    }

    pub fn refresh(&self) -> Buffer {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 200, 200));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        buffer.draw(&self.editor.comp.refresh(), 1, 1);
        buffer
    }
}
