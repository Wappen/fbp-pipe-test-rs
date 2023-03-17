use std::marker::PhantomData;

pub trait SendPipe<T> {
    fn send(&mut self, input: T);
}

pub struct SendPipeImpl<F, T>
where
    F: Fn(T),
{
    on_send: F,
    phantom: PhantomData<T>,
}

impl<F, T> SendPipeImpl<F, T>
where
    F: Fn(T),
{
    pub fn new(on_send: F) -> Self {
        Self {
            on_send,
            phantom: Default::default(),
        }
    }
}

impl<F, T> SendPipe<T> for SendPipeImpl<F, T>
where
    F: Fn(T),
{
    fn send(&mut self, input: T) {
        (self.on_send)(input)
    }
}
