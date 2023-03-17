use crate::pipe::SendPipe;
pub use buffered_transformer::*;
pub use mpsc_transformer::*;
pub use sync_transformer::*;

mod buffered_transformer;
mod mpsc_transformer;
mod sync_transformer;

pub trait Transformer<I, O>: SendPipe<I> {
    fn transform(&mut self, input: I) -> O;
}
