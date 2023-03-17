use crate::forwarder::Forwarder;
use crate::{Pipe, SendPipe};

pub struct SyncForwarder<T> {
    output: Box<dyn SendPipe<T>>,
}

impl<T> SyncForwarder<T> {
    pub fn new(output: Box<dyn SendPipe<T>>) -> Self {
        Self { output }
    }
}

impl<T> Pipe for SyncForwarder<T> {}

impl<T> SendPipe<T> for SyncForwarder<T> {
    fn send(&mut self, input: T) {
        self.forward(input);
    }
}

impl<T> Forwarder<T> for SyncForwarder<T> {
    fn forward(&mut self, input: T) {
        self.output.send(input)
    }
}
