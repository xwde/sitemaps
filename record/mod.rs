#[cfg(any(feature = "txt", feature = "xml"))]
mod sitemap;
#[cfg(any(feature = "txt", feature = "xml"))]
pub use sitemap::*;

#[cfg(feature = "xml")]
mod index;
#[cfg(feature = "xml")]
pub use index::*;

pub mod attribute;
#[cfg(feature = "extension")]
pub mod extension;

pub const RECORDS_LIMIT: usize = 50_000;
pub const BYTES_LIMIT: usize = 52_428_800;

use crate::attribute::Location;

pub trait Record: Sized {
    fn new(location: Location) -> Self;
}
