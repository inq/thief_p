mod editor;
mod line_editor;

pub use self::editor::Editor;
pub use self::line_editor::LineEditor;
use util::ResultBox;

def_child!(Window <- Editor);

impl Window {
    pub fn new_editor() -> Window {
        Window::Editor(Editor::new())
    }
}
