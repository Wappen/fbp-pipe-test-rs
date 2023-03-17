use crate::pipe::{RecvPipe, SendPipe};
use std::sync::mpsc::{channel, Receiver, Sender};

pub fn forwarder<T>() -> (MpscForwarderIn<T>, MpscForwarderOut<T>) {
    let (input, output) = channel();
    (MpscForwarderIn(input), MpscForwarderOut(output))
}

pub struct MpscForwarderIn<T>(Sender<T>);

pub struct MpscForwarderOut<T>(Receiver<T>);

impl<T> SendPipe<T> for MpscForwarderIn<T> {
    fn send(&mut self, input: T) {
        self.0.send(input).expect("send input")
    }
}

impl<T> Clone for MpscForwarderIn<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> RecvPipe<T> for MpscForwarderOut<T> {
    fn recv(&mut self) -> T {
        self.0.recv().expect("recv output")
    }
}

impl<T> RecvPipe<Option<T>> for MpscForwarderOut<T> {
    fn recv(&mut self) -> Option<T> {
        self.0.try_recv().ok()
    }
}
