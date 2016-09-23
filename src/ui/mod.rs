pub mod handler;
mod screen;
mod term;

pub fn init() {
    term::smcup();
    println!("{:?}", term::get_size());
}
