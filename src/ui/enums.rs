use term;

#[derive(Clone, Debug)]
pub enum CommandBar {
    Notify(String),
    Navigate(String),
    Shortcut(String),
}

#[derive(Clone, Debug)]
pub enum Request {
    OpenBuffer(String),
    CommandBar(CommandBar),
    // From hq.
    Keyboard(term::Key),
    Resize(usize, usize),
    Single(usize),
    Pair(usize, usize),
    Quit,
}

#[derive(Debug)]
pub enum Response {
    OpenBuffer(String),
    Command(String),
    Unhandled,
    Quit,
    Term {
        // TODO: Render
        refresh: Option<term::Refresh>,
        cursor: Option<term::Cursor>,
    },
    None,
}

impl Response {
    #[inline]
    pub fn is_handled(&self) -> bool {
        if let Response::Unhandled = *self {
            false
        } else {
            true
        }
    }

    pub fn translate(mut self, tx: usize, ty: usize) -> Response {
        if let Response::Term {
                   ref mut refresh,
                   ref mut cursor,
               } = self {
            if let Some(term::Refresh {
                            ref mut x,
                            ref mut y,
                            ..
                        }) = *refresh {
                *x += tx;
                *y += ty;
            }
            if let Some((ref mut x, ref mut y)) = *cursor {
                *x += tx;
                *y += ty;
            }
        }
        self
    }
}
