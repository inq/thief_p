mod editor;

pub use self::editor::Editor;

use msg::event;
use hq::Hq;
use util::ResultBox;
use ui::res::Response;
use ui::comp::{Component, View};

def_child!(Window <- Editor);

impl Window {
    pub fn new_editor() -> Window {
        Window::Editor(Editor::new())
    }
}
