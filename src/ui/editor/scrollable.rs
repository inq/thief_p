use buf;
use term;
use ui;
use ui::line_editor::LineEditor;
use util::ResultBox;

pub trait Scrollable {
    fn line_editor(&self) -> &LineEditor;
    fn y_offset(&self) -> usize;
    fn height(&self) -> usize;
    fn set_y_offset(&mut self, value: usize);
    fn refresh_with_buffer(&mut self, buffer: &mut buf::Buffer) -> ResultBox<ui::Response>;

    /// Calculate the screen's coordinate of the cursor.
    fn translate_cursor(&self, cursor: term::Cursor) -> term::Cursor {
        let x = self.line_editor().translate_cursor(cursor.0);
        let y = if cursor.1 >= self.y_offset() {
            cursor.1 - self.y_offset()
        } else {
            0
        };
        (x, y)
    }

    fn scroll(&mut self, buffer: &mut buf::Buffer) -> bool {
        let cursor = buffer.cursor();
        if cursor.1 < self.y_offset() {
            // Scroll upward
            self.set_y_offset(cursor.1);
            return true;
        }
        if cursor.1 > self.y_offset() + self.height() {
            // Scroll downward
            let height = self.height();
            self.set_y_offset(cursor.1 - height);
            return true;
        }
        false
    }
}
