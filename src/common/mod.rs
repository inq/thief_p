mod key;
mod event;

pub use self::key::Key;
pub use self::event::Event;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pair {
    pub x: usize,
    pub y: usize,
}
