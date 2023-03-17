pub use buffered_async_forwarder::*;
pub use mpsc_forwarder::*;
pub use sync_forwarder::*;

mod buffered_async_forwarder;
mod mpsc_forwarder;
mod sync_forwarder;

pub trait Forwarder<T> {
    fn forward(&mut self, input: T);
}
