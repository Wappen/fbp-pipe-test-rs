use std::marker::PhantomData;

pub trait RecvPipe<T> {
    fn recv(&mut self) -> T;
}

pub struct RecvPipeImpl<F, T>
where
    F: Fn() -> T,
{
    on_recv: F,
    phantom: PhantomData<T>,
}

impl<F, T> RecvPipeImpl<F, T>
where
    F: Fn() -> T,
{
    pub fn new(on_recv: F) -> Self {
        Self {
            on_recv,
            phantom: Default::default(),
        }
    }
}

impl<F, T> RecvPipe<T> for RecvPipeImpl<F, T>
where
    F: Fn() -> T,
{
    fn recv(&mut self) -> T {
        (self.on_recv)()
    }
}
