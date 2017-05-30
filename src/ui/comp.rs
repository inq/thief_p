use hq;
use ui::{self, Theme};
use util::ResultBox;
use term;

pub struct ViewT {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub theme: Theme,
    pub focus: bool,
}

impl Default for ViewT {
    fn default() -> ViewT {
        ViewT {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            theme: Default::default(),
            focus: true,
        }
    }
}

impl ViewT {
    fn update(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }
}

pub trait View {
    fn get_view(&self) -> &ViewT;
    fn get_view_mut(&mut self) -> &mut ViewT;
}

pub trait Component: View {
    fn on_resize(&mut self, workspace: &mut hq::Workspace) -> ResultBox<()>;
    fn refresh(&mut self, workspace: &mut hq::Workspace) -> ResultBox<ui::Response>;

    /// True iff the component has the focus.
    #[inline]
    fn focus(&self) -> bool {
        self.get_view().focus
    }

    /// Set the focus.
    #[inline]
    fn set_focus(&mut self, value: bool) {
        self.get_view_mut().focus = value;
    }

    /// Resize the component; Call the on_resize function.
    fn resize(&mut self,
              workspace: &mut hq::Workspace,
              x: usize,
              y: usize,
              width: usize,
              height: usize)
              -> ResultBox<()> {
        self.get_view_mut().update(x, y, width, height);
        self.on_resize(workspace)
    }

    /// Propagate if the event is not handled.
    fn unhandled(&mut self, _: &mut hq::Workspace, _: ui::Request) -> ResultBox<ui::Response> {
        Ok(ui::Response::None)
    }

    /// Handle the keyboard event.
    fn on_key(&mut self, _: &mut hq::Workspace, _: term::Key) -> ResultBox<ui::Response> {
        Ok(ui::Response::Unhandled)
    }

    /// Handle the given event.
    fn handle(&mut self, _: &mut hq::Workspace, _: ui::Request) -> ResultBox<ui::Response> {
        Ok(ui::Response::Unhandled)
    }

    /// Propage event to children. This calls handle, and then translate.
    fn propagate(&mut self,
                 e: ui::Request,
                 workspace: &mut hq::Workspace)
                 -> ResultBox<ui::Response> {
        let mut res = if let ui::Request::Keyboard(k) = e {
            self.on_key(workspace, k)?
        } else {
            self.handle(workspace, e.clone())?
        };
        if !res.is_handled() {
            res = self.unhandled(workspace, e)?;
        }
        Ok(res.translate(self.get_view().x, self.get_view().y))
    }
}

pub trait Parent {
    type Child: Component;
    fn children_mut(&mut self) -> Vec<&mut Self::Child>;
    fn children(&self) -> Vec<&Self::Child>;

    /// Draw the children and transform each sequenced results.
    fn refresh_children(&mut self,
                        rect: term::Rect,
                        workspace: &mut hq::Workspace)
                        -> ResultBox<ui::Response> {
        let mut res_refresh = term::Refresh {
            x: 0,
            y: 0,
            rect: rect,
        };
        let mut res_cursor = None;
        for ref mut child in self.children_mut() {
            if let ui::Response::Term { refresh, cursor } = child.refresh(workspace)? {
                if child.focus() {
                    if let Some(cur) = cursor {
                        res_cursor = Some((child.get_view().x + cur.0, child.get_view().y + cur.1));
                    }
                }
                if let Some(term::Refresh { x, y, rect }) = refresh {
                    res_refresh
                        .rect
                        .draw(&rect, child.get_view().x + x, child.get_view().y + y);
                }
            }
        }
        Ok(ui::Response::Term {
               refresh: Some(res_refresh),
               cursor: res_cursor,
           })
    }
}
