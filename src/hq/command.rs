use util::ResultBox;
use hq::Hq;

pub struct Command {
    name: String,
    args: Vec<String>,
    pub func: fn(&mut Hq, &str) -> ResultBox<String>,
}

impl Command {
    // Create a new command. It must be done at the initialization phase.
    pub fn new(name: &str,
               args: Vec<String>,
               func: fn(&mut Hq, &str) -> ResultBox<String>)
               -> Command {
        Command {
            name: String::from(name),
            args: args,
            func: func,
        }
    }
}
