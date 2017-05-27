use hq::Hq;
use msg::event;
use ui::Theme;
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
    fn on_resize(&mut self, hq: &mut Hq) -> ResultBox<()>;
    fn refresh(&mut self, hq: &mut Hq) -> ResultBox<term::Response>;

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
              hq: &mut Hq,
              x: usize,
              y: usize,
              width: usize,
              height: usize)
              -> ResultBox<()> {
        self.get_view_mut().update(x, y, width, height);
        self.on_resize(hq)
    }

    /// Propagate if the event is not handled.
    fn unhandled(&mut self, _: &mut Hq, _: event::Event) -> ResultBox<term::Response> {
        Ok(Default::default())
    }

    /// Handle the keyboard event.
    fn on_key(&mut self, _: &mut Hq, _: event::Key) -> ResultBox<term::Response> {
        Ok(term::Response::Unhandled)
    }

    /// Handle the given event.
    fn handle(&mut self, _: &mut Hq, _: event::Event) -> ResultBox<term::Response> {
        Ok(term::Response::Unhandled)
    }

    /// Propage event to children. This calls handle, and then translate.
    fn propagate(&mut self, e: event::Event, hq: &mut Hq) -> ResultBox<term::Response> {
        let mut res = if let event::Event::Keyboard(k) = e {
            self.on_key(hq, k)?
        } else {
            self.handle(hq, e.clone())?
        };
        if !res.is_handled() {
            res = self.unhandled(hq, e)?;
        }
        Ok(res.translate(self.get_view().x, self.get_view().y))
    }
}

pub trait Parent {
    type Child: Component;
    fn children_mut(&mut self) -> Vec<&mut Self::Child>;
    fn children(&self) -> Vec<&Self::Child>;

    /// Draw the children and transform each sequenced results.
    fn refresh_children(&mut self, rect: term::Rect, hq: &mut Hq) -> ResultBox<term::Response> {
        let mut res_refresh = term::Refresh {
            x: 0,
            y: 0,
            rect: rect,
        };
        let mut res_cursor = None;
        for ref mut child in self.children_mut() {
            if let term::Response::Term { refresh, cursor } = child.refresh(hq)? {
                if child.focus() {
                    if let Some(cur) = cursor {
                        res_cursor = Some(term::Cursor {
                                              x: child.get_view().x + cur.x,
                                              y: child.get_view().y + cur.y,
                                          });
                    }
                }
                if let Some(term::Refresh { x, y, rect }) = refresh {
                    res_refresh
                        .rect
                        .draw(&rect, child.get_view().x + x, child.get_view().y + y);
                }
            }
        }
        Ok(term::Response::Term {
               refresh: Some(res_refresh),
               cursor: res_cursor,
           })
    }
}
