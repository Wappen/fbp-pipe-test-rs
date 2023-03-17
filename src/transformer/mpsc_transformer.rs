use crate::pipe::{Pipe, RecvPipe, SendPipe};
use crate::transformer::Transformer;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct MpscTransformer<I, O> {
    input: Sender<I>,
    output: Receiver<I>,
    transform: Box<dyn Fn(I) -> O>,
}

impl<I, O> MpscTransformer<I, O> {
    pub fn new(transform: Box<dyn Fn(I) -> O>) -> Self {
        let (input, output) = channel();
        Self {
            input,
            output,
            transform,
        }
    }
}

impl<I, O> Pipe for MpscTransformer<I, O> {}

impl<I, O> SendPipe<I> for MpscTransformer<I, O> {
    fn send(&mut self, input: I) {
        self.input.send(input).expect("send input");
    }
}

impl<I, O> RecvPipe<O> for MpscTransformer<I, O> {
    fn recv(&mut self) -> O {
        self.transform(self.output.recv().expect("recv output"))
    }
}

impl<I, O> RecvPipe<Option<O>> for MpscTransformer<I, O> {
    fn recv(&mut self) -> Option<O> {
        self.output.try_recv().ok().map(|v| self.transform(v))
    }
}

impl<I, O> Transformer<I, O> for MpscTransformer<I, O> {
    fn transform(&mut self, input: I) -> O {
        (self.transform)(input)
    }
}
