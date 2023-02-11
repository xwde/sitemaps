mod entry;
mod index;

pub use entry::*;
pub use index::*;

pub const RECORDS_LIMIT: usize = 50_000;
pub const BYTES_LIMIT: usize = 52_428_800;
