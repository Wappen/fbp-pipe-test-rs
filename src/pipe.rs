pub trait Pipe {}

pub trait SendPipe<T>: Pipe {
    fn send(&mut self, arg: T);
}

pub trait RecvPipe<T>: Pipe {
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

impl<T> Pipe for RecvPipeImpl<T> {}

pub struct SendPipeImpl<T> {
    pub on_send: Box<dyn Fn(T)>,
}

impl<T> SendPipe<T> for SendPipeImpl<T> {
    fn send(&mut self, arg: T) {
        (self.on_send)(arg)
    }
}

impl<T> Pipe for SendPipeImpl<T> {}
