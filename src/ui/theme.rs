use ui::res::{Brush, Color};

pub struct Theme {
    pub editor: Brush,
    pub linenum: Brush,
    editor_cur_bg: Color,
    linenum_cur_bg: Color,
}

impl Theme {
    #[inline]
    pub fn editor_cur(&self) -> Brush {
        Brush { bg: self.editor_cur_bg, ..self.editor }
    }

    #[inline]
    pub fn linenum_cur(&self) -> Brush {
        Brush { bg: self.linenum_cur_bg, ..self.linenum }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            editor: Brush::new(Color::new(200, 200, 200), Color::new(40, 40, 40)),
            linenum: Brush::new(Color::new(200, 200, 200), Color::new(80, 80, 80)),
            editor_cur_bg: Color::new(60, 60, 70),
            linenum_cur_bg: Color::new(100, 100, 110),
        }
    }
}
