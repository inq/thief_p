use std::{error, thread};
use std::sync::mpsc;

use io::Event;
use ui::screen::Screen;
use ui::comp::Component;
use ui::res::{Brush, Color, Response};

struct Handler {
    screen: Screen,
}

impl Handler {
    pub fn new(width: usize, height: usize) -> Handler {
        Handler { screen: Screen::new(width, height) }
    }

    pub fn handle(&mut self, e: Event) -> Vec<Response> {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(width, height);
                self.screen.refresh()
            }
            Event::Char { c: x } => vec![Response::Put(format!("{}", x))],
            _ => vec![],
        }
    }
}

fn do_loop(chan_input: &mpsc::Receiver<Event>,
           chan_output: &mpsc::Sender<Vec<Response>>,
           handler: &mut Handler)
           -> Result<(), Box<error::Error>> {
    let event = try!(chan_input.recv());

    try!(chan_output.send(handler.handle(event)));
    // for resp in handler.handle(event) {
    // match resp {
    // Response::Refresh(b) => {
    // b.print(&mut buf, &br.invert());
    // },
    // Response::Move(c) => {
    // term::movexy(&mut buf, c.x, c.y);
    // },
    // Response::Put(s) => {
    // buf.push_str(&s);
    // }
    // }
    // }
    // try!(chan_output.send(buf));
    //
    Ok(())
}

pub fn launch(width: usize, height: usize) -> (mpsc::Sender<Event>, mpsc::Receiver<Vec<Response>>) {
    let (m_event, u_event) = mpsc::channel();
    let (u_response, m_response) = mpsc::channel();
    thread::spawn(move || {
        let mut handler = Handler::new(width, height);
        loop {
            match do_loop(&u_event, &u_response, &mut handler) {
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    });
    (m_event, m_response)
}
