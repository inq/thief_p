use libc;

def_error! {
    SigErr: "signal returned -1",
}

pub fn init() -> Result<(), Error> {
    let res = unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) };
    if res == libc::SIG_ERR {
        return Err(Error::SigErr);
    }
    Ok(())
}
