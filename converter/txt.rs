use crate::attribute::AsUnderlying;
use crate::record::SitemapRecord;
use crate::ConvertSitemap;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct TxtError;

impl Display for TxtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for TxtError {}

#[derive(Debug, Clone, Default)]
pub struct TxtConverter {}

impl TxtConverter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConvertSitemap for TxtConverter {
    type DeError = TxtError;
    type SeError = TxtError;

    /// Deserializes the txt file into the list of records.
    ///
    /// ``` rust
    /// # use sitemaps::{ConvertSitemap, TxtConverter};
    /// let converter = TxtConverter::default();
    /// let sitemap = r#"
    ///     https://www.example.com/example
    ///     https://www.example.com/sample
    /// "#;
    ///
    /// let records = converter.deserialize(sitemap).unwrap();
    /// let records = records.collect::<Vec<_>>();
    ///
    /// assert_eq!(records.len(), 2);
    /// ```
    fn deserialize<'de>(
        &self,
        sitemap: &'de str,
    ) -> Result<impl Iterator<Item = SitemapRecord> + 'de, Self::DeError> {
        Ok(sitemap
            .lines()
            .map(|e| e.trim())
            .map(SitemapRecord::parse)
            .filter_map(|e| e.ok()))
    }

    /// Serializes the list of records into the txt file.
    ///
    /// ```rust
    /// # use sitemaps::{ConvertSitemap, TxtConverter};
    /// # use sitemaps::record::SitemapRecord;
    /// let converter = TxtConverter::default();
    /// let records = vec![
    ///     SitemapRecord::parse("https://www.example.com/example").unwrap(),
    ///     SitemapRecord::parse("https://www.example.com/sample").unwrap(),
    /// ];
    ///
    /// let sitemap = converter.serialize(records.iter()).unwrap();
    /// ```
    fn serialize<'se>(
        &self,
        records: impl Iterator<Item = &'se SitemapRecord>,
    ) -> Result<String, Self::SeError> {
        Ok(records
            .map(|e| e.location.as_underlying().to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n")
    }
}
