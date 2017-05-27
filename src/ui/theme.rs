use term;

pub struct Theme {
    pub editor: term::Brush,
    pub linenum: term::Brush,
    editor_cur_bg: term::Color,
    linenum_cur_bg: term::Color,
}

impl Theme {
    #[inline]
    pub fn editor_cur(&self) -> term::Brush {
        term::Brush {
            bg: self.editor_cur_bg,
            ..self.editor
        }
    }

    #[inline]
    pub fn linenum_cur(&self) -> term::Brush {
        term::Brush {
            bg: self.linenum_cur_bg,
            ..self.linenum
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            editor: term::Brush::new(term::Color::new(200, 200, 200),
                                     term::Color::new(40, 40, 40)),
            linenum: term::Brush::new(term::Color::new(200, 200, 200),
                                      term::Color::new(80, 80, 80)),
            editor_cur_bg: term::Color::new(60, 60, 70),
            linenum_cur_bg: term::Color::new(100, 100, 110),
        }
    }
}
