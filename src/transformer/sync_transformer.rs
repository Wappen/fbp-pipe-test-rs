use crate::transformer::Transformer;
use crate::{Pipe, SendPipe};

pub struct SyncTransformer<I, O> {
    transform: Box<dyn Fn(I) -> O>,
    output: Box<dyn SendPipe<O>>,
}

impl<I, O> SyncTransformer<I, O> {
    pub fn new(transform: Box<dyn Fn(I) -> O>, output: Box<dyn SendPipe<O>>) -> Self {
        Self { transform, output }
    }
}

impl<I, O> Pipe for SyncTransformer<I, O> {}

impl<I, O> SendPipe<I> for SyncTransformer<I, O> {
    fn send(&mut self, input: I) {
        let result = self.transform(input);
        self.output.send(result);
    }
}

impl<I, O> Transformer<I, O> for SyncTransformer<I, O> {
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
