#[macro_use]
mod macros;

mod chan;

pub use util::chan::Chan;

pub type ResultBox<T> = ::std::result::Result<T, Box<::std::error::Error>>;
