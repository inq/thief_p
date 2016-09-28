use std::sync::mpsc;

pub struct Chan<R, S> {
    receiver: mpsc::Receiver<R>,
    sender: mpsc::Sender<S>,
}

impl<R, S> Chan<R, S> {
    pub fn create() -> (Chan<R, S>, Chan<S, R>) {
        let (s1, r1) = mpsc::channel();
        let (s2, r2) = mpsc::channel();
        (Chan {
            receiver: r1,
            sender: s2,
        },
         Chan {
            receiver: r2,
            sender: s1,
        })
    }

    pub fn recv(&self) -> Result<R, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<R, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn send(&self, content: S) -> Result<(), mpsc::SendError<S>> {
        self.sender.send(content)
    }
}
