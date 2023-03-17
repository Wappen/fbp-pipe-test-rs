use crate::pipe::{RecvPipe, SendPipe};
use crate::transformer::Transformer;

pub struct BufferedTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    buffer: Option<I>,
    transform: F,
}

impl<F, I, O> BufferedTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    pub fn new(transform: F) -> Self {
        Self {
            buffer: None,
            transform,
        }
    }
}

impl<F, I, O> SendPipe<I> for BufferedTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn send(&mut self, input: I) {
        self.buffer = Some(input);
    }
}

impl<F, I, O> RecvPipe<Option<O>> for BufferedTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn recv(&mut self) -> Option<O> {
        self.buffer.take().map(|val| self.transform(val))
    }
}

impl<F, I, O> Transformer<I, O> for BufferedTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
