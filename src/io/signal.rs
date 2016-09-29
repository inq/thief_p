use libc;

def_error! {
    Initialized: "already initialized",
    SigErr: "signal returned -1",
}

pub fn init() -> Result<()> {
    allow_once!();

    let res = unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) };
    if res == libc::SIG_ERR {
        return Err(Error::SigErr);
    }
    Ok(())
}
