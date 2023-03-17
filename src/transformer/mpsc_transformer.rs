use crate::pipe::{RecvPipe, SendPipe};
use std::sync::mpsc::{channel, Receiver, Sender};

pub fn transformer<F, I, O>(transform: F) -> (MpscTransformerIn<I>, MpscTransformerOut<F, I, O>)
where
    F: Fn(I) -> O,
{
    let (input, output) = channel();
    (
        MpscTransformerIn(input),
        MpscTransformerOut { output, transform },
    )
}

pub struct MpscTransformerIn<T>(Sender<T>);

pub struct MpscTransformerOut<F, I, O>
where
    F: Fn(I) -> O,
{
    output: Receiver<I>,
    transform: F,
}

impl<T> SendPipe<T> for MpscTransformerIn<T> {
    fn send(&mut self, input: T) {
        self.0.send(input).expect("send input");
    }
}

impl<F, I, O> RecvPipe<O> for MpscTransformerOut<F, I, O>
where
    F: Fn(I) -> O,
{
    fn recv(&mut self) -> O {
        (self.transform)(self.output.recv().expect("recv output"))
    }
}

impl<F, I, O> RecvPipe<Option<O>> for MpscTransformerOut<F, I, O>
where
    F: Fn(I) -> O,
{
    fn recv(&mut self) -> Option<O> {
        self.output.try_recv().ok().map(|v| (self.transform)(v))
    }
}
