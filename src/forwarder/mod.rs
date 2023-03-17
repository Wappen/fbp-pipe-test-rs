use crate::pipe::{RecvPipe, SendPipe};
pub use buffered_forwarder::*;
pub use mpsc_forwarder::*;

mod buffered_forwarder;
mod mpsc_forwarder;

pub trait Forwarder<T>: SendPipe<T> + RecvPipe<T> {
    fn forward(&mut self, input: T);
}
