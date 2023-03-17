pub trait SendPipe<T> {
    fn send(&mut self, input: T);
}

pub struct SendPipeImpl<T> {
    pub on_send: Box<dyn Fn(T)>,
}

impl<T> SendPipe<T> for SendPipeImpl<T> {
    fn send(&mut self, input: T) {
        (self.on_send)(input)
    }
}
