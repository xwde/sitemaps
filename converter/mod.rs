use crate::record::{IndexRecord, SitemapRecord};
use std::error::Error;

mod txt;
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

pub trait ConvertSitemap {
    type DeError: Error;
    type SeError: Error;

    fn deserialize<'de>(
        &self,
        sitemap: &'de str,
    ) -> Result<impl Iterator<Item = SitemapRecord> + 'de, Self::DeError>;
    fn serialize<'se>(
        &self,
        records: impl Iterator<Item = &'se SitemapRecord>,
    ) -> Result<String, Self::SeError>;
}

pub trait ConvertIndex {
    type DeError: Error;
    type SeError: Error;

    fn deserialize_index<'de>(
        &self,
        index: &'de str,
    ) -> Result<impl Iterator<Item = IndexRecord> + 'de, Self::DeError>;
    fn serialize_index<'se>(
        &self,
        records: impl Iterator<Item = &'se IndexRecord>,
    ) -> Result<String, Self::SeError>;
}

// pub trait ConvertAuto {
//     fn deserialize(&self) -> Result<(), ()>;
//     fn serialize(&self) -> Result<(), ()>;
// }
