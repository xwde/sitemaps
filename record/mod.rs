pub mod attribute;
#[cfg(feature = "extension")]
pub mod extension;

#[cfg(any(feature = "txt", feature = "xml"))]
mod sitemap;
#[cfg(any(feature = "txt", feature = "xml"))]
pub use sitemap::*;

#[cfg(feature = "xml")]
mod index;
#[cfg(feature = "xml")]
pub use index::*;

pub mod limits {
    pub const RECORDS_LIMIT: usize = 50_000;
    pub const BYTES_LIMIT: usize = 52_428_800;
    pub const RESOURCE_LENGTH_LIMIT: usize = 2048;
}

use crate::attribute::Location;

pub trait Record: Sized {
    fn new(location: Location) -> Self;
}
