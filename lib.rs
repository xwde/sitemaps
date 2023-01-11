#![forbid(unsafe_code)]

#[cfg(feature = "reader")]
pub use reader::*;
#[cfg(feature = "writer")]
pub use writer::*;

#[cfg(any(feature = "reader", feature = "writer"))]
pub mod attributes;
#[cfg(any(feature = "reader", feature = "writer"))]
pub mod entries;
#[cfg(feature = "reader")]
mod reader;
#[cfg(feature = "writer")]
mod writer;

#[cfg(not(any(feature = "reader", feature = "writer")))]
compile_error!("one of features ['reader', 'writer'] must be enabled");
