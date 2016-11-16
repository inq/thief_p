mod editor;
mod line_number;
mod edit;

pub use self::editor::Editor;
pub use self::line_number::LineNumber;
pub use self::edit::Edit;

use io::Event;
use ui::res::{Response};
use ui::comp::{Component, View};

pub enum Window {
    Edit(Edit)
}

impl Window {
    pub fn new_edit() -> Window {
        Window::Edit(Edit::new())
    }
}

impl Component for Window {
    fn get_view(&self) -> &View {
        match *self {
            Window::Edit(ref ew) => ew.get_view(),
        }
    }

    fn get_view_mut(&mut self) -> &mut View {
        match *self {
            Window::Edit(ref mut ew) => ew.get_view_mut(),
        }
    }

    fn on_resize(&mut self) {
        match *self {
            Window::Edit(ref mut ew) => ew.on_resize(),
        }
    }

    fn refresh(&self) -> Response {
        match *self {
            Window::Edit(ref ew) => ew.refresh(),
        }
    }

    fn handle(&mut self, e: Event) -> Response {
        match *self {
            Window::Edit(ref mut ew) => ew.handle(e),
        }
    }
}
