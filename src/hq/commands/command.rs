use hq::{Arg, Func};

#[allow(dead_code)]
pub struct Command {
    name: String,
    args: Vec<Arg>,
    pub func: Func,
}

impl Command {
    /// Create a new command. It must be done at the initialization phase.
    pub fn new(name: &str, args: Vec<Arg>, func: Func) -> Command {
        Command {
            name: String::from(name),
            args: args,
            func: func,
        }
    }

    pub fn args_len(&self) -> usize {
        self.args.len()
    }

    pub fn arg(&self, idx: usize) -> Arg {
        self.args[idx].clone()
    }
}
