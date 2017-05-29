use util::ResultBox;
use hq::workspace::Workspace;
use msg::event::Event;

pub type Func = fn(&mut Workspace, Vec<String>) -> ResultBox<Event>;

#[derive(Clone)]
pub enum Arg {
    Path(String),
//    String(String),
}
