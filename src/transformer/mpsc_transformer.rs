use crate::pipe::{RecvPipe, SendPipe};
use crate::transformer::Transformer;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    input: Sender<I>,
    output: Receiver<I>,
    transform: F,
}

impl<F, I, O> MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    pub fn new(transform: F) -> Self {
        let (input, output) = channel();
        Self {
            input,
            output,
            transform,
        }
    }
}

impl<F, I, O> SendPipe<I> for MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn send(&mut self, input: I) {
        self.input.send(input).expect("send input");
    }
}

impl<F, I, O> RecvPipe<O> for MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn recv(&mut self) -> O {
        self.transform(self.output.recv().expect("recv output"))
    }
}

impl<F, I, O> RecvPipe<Option<O>> for MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn recv(&mut self) -> Option<O> {
        self.output.try_recv().ok().map(|v| self.transform(v))
    }
}

impl<F, I, O> Transformer<I, O> for MpscTransformer<F, I, O>
where
    F: Fn(I) -> O,
{
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
