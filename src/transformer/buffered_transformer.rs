use crate::pipe::{RecvPipe, SendPipe};
use crate::transformer::Transformer;

pub struct BufferedTransformer<I, O> {
    buffer: Option<Box<I>>,
    transform: Box<dyn Fn(I) -> O>,
}

impl<I, O> BufferedTransformer<I, O> {
    pub fn new(transform: Box<dyn Fn(I) -> O>) -> Self {
        Self {
            buffer: None,
            transform,
        }
    }
}

impl<I, O> SendPipe<I> for BufferedTransformer<I, O> {
    fn send(&mut self, input: I) {
        self.buffer = Some(Box::new(input));
    }
}

impl<I, O> RecvPipe<Option<O>> for BufferedTransformer<I, O> {
    fn recv(&mut self) -> Option<O> {
        self.buffer.take().map(|val| self.transform(*val))
    }
}

impl<I, O> Transformer<I, O> for BufferedTransformer<I, O> {
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
