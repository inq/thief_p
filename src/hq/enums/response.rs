use term;

pub enum Response {
    // TODO: Term
    //Command(String),
    Quit,
    Term {
        // TODO: Render
        refresh: Option<term::Refresh>,
        cursor: Option<term::Cursor>,
    },
    None,
}
