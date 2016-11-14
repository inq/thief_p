mod line;
mod buffer;

pub use self::buffer::Buffer;
pub use self::line::Line;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
