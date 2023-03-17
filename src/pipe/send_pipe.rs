use crate::pipe::Pipe;

pub trait SendPipe<T>: Pipe {
    fn send(&mut self, arg: T);
}

pub struct SendPipeImpl<T> {
    pub on_send: Box<dyn Fn(T)>,
}

impl<T> SendPipe<T> for SendPipeImpl<T> {
    fn send(&mut self, arg: T) {
        (self.on_send)(arg)
    }
}

impl<T> Pipe for SendPipeImpl<T> {}
