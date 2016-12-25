use msg::event::Event;
use util::ResultBox;
use hq::Hq;
use hq::workspace::Workspace;

pub type Func = fn(&mut Workspace, Vec<String>) -> ResultBox<Event>;

pub enum Arg {
    Path(String),
    String(String),
}

pub struct Command {
    name: String,
    args: Vec<Arg>,
    pub func: Func,
}

impl Command {
    /// Create a new command. It must be done at the initialization phase.
    pub fn new(name: &str,
               args: Vec<Arg>,
               func: Func)
               -> Command {
        Command {
            name: String::from(name),
            args: args,
            func: func,
        }
    }

    pub fn args_len(&self) -> usize {
        self.args.len()
    }
}
