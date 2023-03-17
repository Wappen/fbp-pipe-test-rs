pub use buffered_async_transformer::*;
pub use mpsc_transformer::*;
pub use sync_transformer::*;

mod buffered_async_transformer;
mod mpsc_transformer;
mod sync_transformer;

pub trait Transformer<I, O> {
    fn transform(&mut self, input: I) -> O;
}
