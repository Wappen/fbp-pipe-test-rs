use crate::pipe::SendPipe;
use crate::transformer::Transformer;
use std::marker::PhantomData;

pub struct SyncTransformer<F, U, I, O>
where
    F: Fn(I) -> O,
    U: SendPipe<O>,
{
    transform: F,
    output: U,
    phantom: PhantomData<I>,
}

impl<F, U, I, O> SyncTransformer<F, U, I, O>
where
    F: Fn(I) -> O,
    U: SendPipe<O>,
{
    pub fn new(transform: F, output: U) -> Self {
        Self {
            transform,
            output,
            phantom: Default::default(),
        }
    }
}

impl<F, U, I, O> SendPipe<I> for SyncTransformer<F, U, I, O>
where
    F: Fn(I) -> O,
    U: SendPipe<O>,
{
    fn send(&mut self, input: I) {
        let result = self.transform(input);
        self.output.send(result);
    }
}

impl<F, U, I, O> Transformer<I, O> for SyncTransformer<F, U, I, O>
where
    F: Fn(I) -> O,
    U: SendPipe<O>,
{
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
