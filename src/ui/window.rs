use ui::editor::Editor;
use ui::prim::{Buffer, Brush, Color};

pub struct Window {
    editor: (Editor, usize, usize),
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        Window {
            editor: (Editor::new(width, height), 1, 1),
            width: width,
            height: height
        }
    }
}

impl Window {
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.editor.0.resize(width - 2, height - 2);
    }

    pub fn refresh(&self) -> Buffer {
        let b = Brush::new(Color::new(0, 0, 0), Color::new(200, 200, 200));
        let mut buffer = Buffer::blank(&b, self.width, self.height);
        buffer.draw(&self.editor.0.refresh(), 1, 1);
        buffer
    }
}
