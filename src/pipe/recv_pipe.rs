pub trait RecvPipe<T> {
    fn recv(&mut self) -> T;
}

pub struct RecvPipeImpl<T> {
    pub on_recv: Box<dyn Fn() -> T>,
}

impl<T> RecvPipe<T> for RecvPipeImpl<T> {
    fn recv(&mut self) -> T {
        (self.on_recv)()
    }
}
