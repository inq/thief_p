pub enum Response {
    Something,
    Nothing,
}

pub fn query(ipt: &str) -> Response {
    if ipt == "command" {
        Response::Something
    } else {
        Response::Nothing
    }
}
