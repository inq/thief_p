use std::{error, thread};
use std::sync::mpsc;

use io::Event;
use ui::screen::Screen;

struct Handler {
    screen: Screen,
}

impl Handler {
    pub fn new(width: usize, height: usize) -> Handler {
        Handler { screen: Screen::new(width, height) }
    }

    pub fn handle(&mut self, mut buf: &mut String, e: Event) -> Result<(), Box<error::Error>> {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(width, height);
                try!(self.screen.refresh(&mut buf));
            }
            Event::Char { c: x } => {
                buf.push_str(&format!("{}", x));
                try!(self.screen.refresh(&mut buf));
            }
            _ => (),
        }
        Ok(())
    }
}

fn do_loop(chan_input: &mpsc::Receiver<Event>,
           chan_output: &mpsc::Sender<String>,
           handler: &mut Handler)
           -> Result<(), Box<error::Error>> {
    let res = try!(chan_input.recv());
    let mut buf = String::with_capacity(4096);
    try!(handler.handle(&mut buf, res));
    try!(chan_output.send(buf));

    Ok(())
}

pub fn launch(width: usize, height: usize)
              -> (mpsc::Sender<Event>, mpsc::Receiver<String>) {
    let (m_event, u_event) = mpsc::channel();
    let (u_string, m_string) = mpsc::channel();
    thread::spawn(move || {
        let mut handler = Handler::new(width, height);
        loop {
            match do_loop(&u_event, &u_string, &mut handler) {
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    });
    (m_event, m_string)
}
