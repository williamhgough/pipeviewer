pub mod args;
pub mod read;
pub mod stats;
pub mod write;

/// CHUNK_SIZE is an arbitrary chunk size for the buffer.
const CHUNK_SIZE: usize = 16 * 1024; // 16Kb
