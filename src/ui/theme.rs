use ui::res::{Brush, Color};

pub struct Theme {
    pub editor: Brush,
    pub linenum: Brush,
    pub editor_cur_bg: Color,
    pub linenum_cur_bg: Color,
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
