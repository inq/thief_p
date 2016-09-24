use std::thread;
use std::sync::mpsc::{self, channel};
use io::Event;
use ui::color::{Brush, Color};
use ui::char::Char;
use ui::line::Line;

struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn handle(&self, e: Event) {
        let b = Brush::new(
            Color::new(0, 0, 0),
            Color::new(200, 250, 250),
        );
        match e {
            Event::Char{c: x} => {
                let c = Char { chr: x, brush: b.clone() };
                println!("{}", c.width());
                println!("{:?}", c);

                let line = Line::blank(&b, 16);
                line.print(&Brush::new(Color::new(1, 1, 1), Color::new(1, 1, 1)));
            },
            _ => ()
        }
    }
}


pub fn launch() -> mpsc::Sender<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let handler = Handler::new();
        loop {
            let res = rx.recv().unwrap();
            handler.handle(res);
        }
    });
    tx
}
