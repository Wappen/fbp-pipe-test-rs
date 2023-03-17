use crate::forwarder::Forwarder;
use crate::{Pipe, RecvPipe, SendPipe};

#[derive(Default)]
pub struct BufferedAsyncForwarder<T> {
    buffer: Option<Box<T>>,
}

impl<T> Pipe for BufferedAsyncForwarder<T> {}

impl<T> SendPipe<T> for BufferedAsyncForwarder<T> {
    fn send(&mut self, input: T) {
        self.forward(input)
    }
}

impl<T> RecvPipe<Option<T>> for BufferedAsyncForwarder<T> {
    fn recv(&mut self) -> Option<T> {
        self.buffer.take().map(|v| *v)
    }
}

impl<T> Forwarder<T> for BufferedAsyncForwarder<T> {
    fn forward(&mut self, input: T) {
        self.buffer = Some(Box::new(input));
    }
}
