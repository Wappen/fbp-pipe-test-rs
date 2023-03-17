use crate::transformer::Transformer;
use crate::{Pipe, RecvPipe, SendPipe};

pub struct BufferedAsyncTransformer<I, O> {
    buffer: Option<Box<I>>,
    transform: Box<dyn Fn(I) -> O>,
}

impl<I, O> BufferedAsyncTransformer<I, O> {
    pub fn new(transform: Box<dyn Fn(I) -> O>) -> Self {
        Self {
            buffer: None,
            transform,
        }
    }
}

impl<I, O> Pipe for BufferedAsyncTransformer<I, O> {}

impl<I, O> SendPipe<I> for BufferedAsyncTransformer<I, O> {
    fn send(&mut self, input: I) {
        self.buffer = Some(Box::new(input));
    }
}

impl<I, O> RecvPipe<Option<O>> for BufferedAsyncTransformer<I, O> {
    fn recv(&mut self) -> Option<O> {
        self.buffer.take().map(|val| self.transform(*val))
    }
}

impl<I, O> Transformer<I, O> for BufferedAsyncTransformer<I, O> {
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
