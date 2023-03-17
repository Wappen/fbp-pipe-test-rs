use crate::forwarder::Forwarder;
use crate::pipe::{Pipe, RecvPipe, SendPipe};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct MpscForwarder<T> {
    input: Sender<T>,
    output: Receiver<T>,
}

impl<T> Default for MpscForwarder<T> {
    fn default() -> Self {
        let (input, output) = channel();
        Self { input, output }
    }
}

impl<T> Pipe for MpscForwarder<T> {}

impl<T> SendPipe<T> for MpscForwarder<T> {
    fn send(&mut self, input: T) {
        self.forward(input);
    }
}

impl<T> RecvPipe<T> for MpscForwarder<T> {
    fn recv(&mut self) -> T {
        self.output.recv().expect("recv output")
    }
}

impl<T> RecvPipe<Option<T>> for MpscForwarder<T> {
    fn recv(&mut self) -> Option<T> {
        self.output.try_recv().ok()
    }
}

impl<T> Forwarder<T> for MpscForwarder<T> {
    fn forward(&mut self, input: T) {
        self.input.send(input).expect("send input")
    }
}
