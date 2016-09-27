use std::{error, thread};
use std::sync::mpsc;

use io::Event;
use ui::screen::Screen;
use ui::comp::Component;
use ui::res::Response;

struct Handler {
    screen: Screen,
    quit: bool,
}

impl Handler {
    pub fn new(width: usize, height: usize) -> Handler {
        Handler { screen: Screen::new(width, height), quit: false }
    }

    pub fn handle(&mut self, e: Event) -> Vec<Response> {
        match e {
            Event::Resize { w: width, h: height } => {
                self.screen.resize(width, height);
                self.screen.refresh()
            }
            Event::Ctrl { c: c } => {
                match c {
                    'q' => {
                        self.quit = true;
                        vec![Response::Quit]
                    },
                    _ => vec![],
                }
            }
            Event::Char { c: c } => vec![Response::Put(format!("{}", c))],
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
    Ok(())
}

pub fn launch(width: usize, height: usize) -> (mpsc::Sender<Event>, mpsc::Receiver<Vec<Response>>) {
    let (m_event, u_event) = mpsc::channel();
    let (u_response, m_response) = mpsc::channel();
    thread::spawn(move || {
        let mut handler = Handler::new(width, height);
        while !handler.quit {
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
