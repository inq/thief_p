mod request;
mod response;

use util::ResultBox;
use ui;
use hq::workspace::Workspace;

pub use self::request::Request;
pub use self::response::Response;

pub type Func = fn(&mut Workspace, Vec<String>) -> ResultBox<ui::Request>;
pub type Pair = (usize, usize);

#[derive(Clone)]
pub enum Arg {
    Path(String),
//    String(String),
}
