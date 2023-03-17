use crate::pipe::{Pipe, RecvPipe, SendPipe};

#[derive(Default)]
pub struct BufferedForwarder<T> {
    buffer: Option<Box<T>>,
}

impl<T> Pipe for BufferedForwarder<T> {}

impl<T> SendPipe<T> for BufferedForwarder<T> {
    fn send(&mut self, input: T) {
        self.buffer = Some(Box::new(input))
    }
}

impl<T> RecvPipe<T> for BufferedForwarder<T> {
    fn recv(&mut self) -> T {
        *self.buffer.take().expect("value in buffer")
    }
}

impl<T> RecvPipe<Option<T>> for BufferedForwarder<T> {
    fn recv(&mut self) -> Option<T> {
        self.buffer.take().map(|v| *v)
    }
}
