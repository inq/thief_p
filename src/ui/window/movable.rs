use buf::Buffer;
use util::ResultBox;
use ui::window::line_editor::LineEditorRes;

pub enum Direction {
    Horizontal(i8),
    Vertical(i8),
}

impl Direction {
    fn dx(&self) -> i8 {
        match *self {
            Direction::Horizontal(dx) => dx,
            _ => 0,
        }
    }

    fn dy(&self) -> i8 {
        match *self {
            Direction::Vertical(dx) => dx,
            _ => 0,
        }
    }
}

pub trait Movable {
    fn x_offset(&self) -> usize;
    fn set_x_offset(&mut self, usize);
    fn increase_x_offset(&mut self, amount: usize);
    fn decrease_x_offset(&mut self, amount: usize);

    /// The width of the object
    fn width(&self) -> usize;
    fn response_cursor(&self, cursor: usize) -> ResultBox<LineEditorRes>;

    /// Adjust x_offset to make sense.
    /// Return true iff the x_offset has been changed.
    fn adjust_x_offset(&mut self, cursor: usize) -> bool {
        if self.x_offset() > cursor {
            self.set_x_offset(cursor);
            return true;
        }
        let width = self.width();
        if cursor > self.x_offset() + width {
            self.set_x_offset(cursor - width);
            return true;
        }
        false
    }

    /// Handle move events.
    fn on_move(&mut self, buf: &mut Buffer, direction: Direction) -> ResultBox<LineEditorRes> {
        let cursor_prev = buf.get_cursor();
        let cursor = buf.move_cursor(direction.dx(), direction.dy());
        if cursor_prev.1 == cursor.1 {
            // Move only in here.
            if cursor.0 - self.x_offset() >= self.width() {
                self.increase_x_offset(1);
                Ok(LineEditorRes::Refresh)
            } else if self.x_offset() > 0 && cursor.0 - self.x_offset() <= 1 {
                self.decrease_x_offset(1);
                Ok(LineEditorRes::Refresh)
            } else {
                self.response_cursor(cursor.0)
            }
        } else {
            // Pass the process to the parrent.
            Ok(LineEditorRes::Move(cursor_prev, cursor))
        }
    }

    /// Handle the HOME(C-a) and END(C-e) key event.
    fn on_home_end(&mut self, buf: &mut Buffer, head: bool) -> ResultBox<LineEditorRes> {
        let cursor = if head {
            buf.move_begin_of_line().0
        } else {
            buf.move_end_of_line().0
        };
        if self.adjust_x_offset(cursor) {
            Ok(LineEditorRes::Refresh)
        } else {
            self.response_cursor(cursor)
        }
    }
}
