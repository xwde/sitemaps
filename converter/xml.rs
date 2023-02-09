use crate::record::{IndexRecord, SitemapRecord};
use crate::{ConvertIndex, ConvertSitemap};

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use sitemaps::attribute::modified::LastModified;

#[derive(Debug, Clone)]
pub struct XmlError;

impl Display for XmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for XmlError {}

#[derive(Debug, Clone, Default)]
pub struct XmlConverter {}

impl XmlConverter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConvertSitemap for XmlConverter {
    type DeError = XmlError;
    type SeError = XmlError;

    /// Deserializes the xml file into the list of records.
    ///
    /// ```rust
    /// # use sitemaps::{ConvertSitemap, XmlConverter};
    /// let converter = XmlConverter::default();
    /// let sitemap = r#"
    ///    <urlset xmlns="https://www.sitemaps.org/schemas/sitemap/0.9">
    ///         <url>
    ///             <loc>https://example.com/</loc>
    ///             <lastmod>2023-01-10T22:11:17.000-05:00</lastmod>
    ///             <changefreq>daily</changefreq>
    ///             <priority>0.8</priority>
    ///         </url>
    ///    </urlset>
    /// "#;
    ///
    /// let records = converter.deserialize(sitemap).unwrap();
    /// let records = records.collect::<Vec<_>>();
    ///
    /// assert_eq!(records.len(), 1);
    /// ```
    fn deserialize<'de>(
        &self,
        sitemap: &'de str,
    ) -> Result<impl Iterator<Item = SitemapRecord> + 'de, Self::DeError> {
        todo!()
    }

    /// Serializes the list of records into the xml file.
    ///
    /// ```rust
    /// # use sitemaps::{ConvertSitemap, XmlConverter};
    /// # use sitemaps::attribute::AsAttribute;
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// # use sitemaps::attribute::location::Location;
    /// # use sitemaps::attribute::modified::LastModified;
    /// # use sitemaps::attribute::priority::Priority;
    /// # use sitemaps::record::SitemapRecord;
    /// let converter = XmlConverter::default();
    /// let records = vec![SitemapRecord {
    ///     location: Location::parse("").unwrap(),
    ///     last_modified: LastModified::parse("2023-01-10T22:11:17.000-05:00").ok(),
    ///     change_frequency: ChangeFrequency::parse("daily").ok(),
    ///     priority: Priority::parse("0.8").ok(),
    /// }];
    ///
    /// let sitemap = converter.serialize(records.iter()).unwrap();
    /// ```
    fn serialize<'se>(
        &self,
        records: impl Iterator<Item = &'se SitemapRecord>,
    ) -> Result<String, Self::SeError> {
        todo!()
    }
}

impl ConvertIndex for XmlConverter {
    type DeError = XmlError;
    type SeError = XmlError;

    /// Deserializes the xml file into the list of records.
    ///
    /// ```rust
    /// # use sitemaps::{ConvertIndex, XmlConverter};
    /// let converter = XmlConverter::default();
    /// let index = r#"
    ///    <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    ///         <sitemap>
    ///             <loc>https://example.com/</loc>
    ///             <lastmod>2023-01-10T22:11:17.000-05:00</lastmod>
    ///         </sitemap>
    ///    </sitemapindex>
    /// "#;
    ///
    /// let records = converter.deserialize_index(index).unwrap();
    /// let records = records.collect::<Vec<_>>();
    ///
    /// assert_eq!(records.len(), 1);
    /// ```
    fn deserialize_index<'de>(
        &self,
        index: &'de str,
    ) -> Result<impl Iterator<Item = IndexRecord> + 'de, Self::DeError> {
        todo!()
    }

    /// Serializes the list of records into the xml file.
    ///
    /// ```rust
    /// # use sitemaps::{ConvertIndex, XmlConverter};
    /// # use sitemaps::attribute::AsAttribute;
    /// # use sitemaps::attribute::location::Location;
    /// # use sitemaps::attribute::modified::LastModified;
    /// # use sitemaps::record::IndexRecord;
    /// let converter = XmlConverter::default();
    /// let records = vec![IndexRecord {
    ///     location: Location::parse("https://example.com/").unwrap(),
    ///     last_modified: LastModified::parse("2023-01-10T22:11:17.000-05:00").ok(),
    /// }];
    ///
    /// let index = converter.serialize_index(records.iter()).unwrap();
    /// ```
    fn serialize_index<'se>(
        &self,
        records: impl Iterator<Item = &'se IndexRecord>,
    ) -> Result<String, Self::SeError> {
        todo!()
    }
}
