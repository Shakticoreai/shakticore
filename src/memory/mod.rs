pub mod pool;
pub mod allocator;
pub mod zero_copy;

pub use pool::MemoryPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Backend error: {0}")]
    BackendError(String),
}
