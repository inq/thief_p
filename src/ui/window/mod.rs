mod editor;
mod edit;

pub use self::editor::Editor;
pub use self::edit::Edit;

use hq::Hq;
use io::Event;
use util::ResultBox;
use ui::res::Response;
use ui::comp::{Component, View};

def_child!(Window <- Edit);

impl Window {
    pub fn new_edit() -> Window {
        Window::Edit(Edit::new())
    }
}