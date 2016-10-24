pub struct Line {
    before: String,
    after: String,
}

impl Line {
    pub fn new() -> Line {
        Line {
            before: String::with_capacity(80),
            after: String::with_capacity(80),
        }
    }

    pub fn to_string(&self) -> String {
        let reversed = self.after.chars().rev().collect::<String>();
        format!("{}{}", self.before, reversed)
    }
}

#[test]
fn basic_operations() {
    let t = Line {
        before: String::from("hello"),
        after: String::from("world"),
    };
    assert_eq!(t.to_string(), "hellodlrow");
}
