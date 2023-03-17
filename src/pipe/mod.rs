pub use recv_pipe::*;
pub use send_pipe::*;

mod recv_pipe;
mod send_pipe;

pub trait Pipe {}
