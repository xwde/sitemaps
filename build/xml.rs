use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Error as XmlError, Writer};

use crate::attribute::AsUnderlying;
use crate::build::{IndexBuilder, SitemapBuilder};
use crate::{IndexRecord, SitemapRecord};

#[derive(Debug)]
pub enum XmlBuilderError {
    TooManyRecords,
    TooManyBytes(usize),
    XmlError(XmlError),
}

impl Display for XmlBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::TooManyRecords => write!(f, "too many records"),
            Self::TooManyBytes(n) => write!(f, "too many bytes: {n}"),
            Self::XmlError(e) => Display::fmt(&e, f),
        }
    }
}

impl From<XmlError> for XmlBuilderError {
    fn from(error: XmlError) -> Self {
        XmlBuilderError::XmlError(error)
    }
}

impl Error for XmlBuilderError {}

/// Xml Sitemap & Index Builder.
///
/// ```rust
/// # use sitemaps::build::{SitemapStringBuilder, XmlBuilder};
/// # use sitemaps::SitemapRecord;
/// let uri = "https://www.example.com/";
/// let record = SitemapRecord::parse(uri).unwrap();
/// let records = vec![record /* & more records... */];
/// let sitemap = XmlBuilder::build_string(records.iter()).unwrap();
/// ```
pub struct XmlBuilder<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> XmlBuilder<W> {
    const XMLNS: [(&'static str, &'static str); 1] =
        [("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9")];

    const URL_SET: &'static str = "urlset";
    const URL: &'static str = "url";

    const SITEMAP_INDEX: &'static str = "sitemapindex";
    const SITEMAP: &'static str = "sitemap";

    const LOCATION: &'static str = "loc";
    const LAST_MODIFIED: &'static str = "lastmod";
    const CHANGE_FREQUENCY: &'static str = "changefreq";
    const PRIORITY: &'static str = "priority";
}

impl<W: Write> SitemapBuilder<W> for XmlBuilder<W> {
    type Error = XmlBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        let mut writer = Writer::new(writer);
        writer.write_bom()?;

        let tag = BytesStart::new(Self::URL_SET);
        let tag = tag.with_attributes(Self::XMLNS);
        writer.write_event(Event::Start(tag))?;

        Ok(Self { writer })
    }

    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        let tag = self.writer.create_element(Self::URL);
        tag.write_inner_content(|writer| {
            {
                let location = record.location.to_string();
                let location = location.as_str();
                let tag = writer.create_element(Self::LOCATION);
                tag.write_text_content(BytesText::new(location))?;
            }

            if let Some(last_modified) = &record.last_modified {
                let last_modified = last_modified.to_string();
                let last_modified = last_modified.as_str();
                let tag = writer.create_element(Self::LAST_MODIFIED);
                tag.write_text_content(BytesText::new(last_modified))?;
            }

            if let Some(change_frequency) = &record.change_frequency {
                let change_frequency = change_frequency.as_underlying();
                let tag = writer.create_element(Self::CHANGE_FREQUENCY);
                tag.write_text_content(BytesText::new(change_frequency))?;
            }

            if let Some(priority) = &record.priority {
                let priority = priority.to_string();
                let priority = priority.as_str();
                let tag = writer.create_element(Self::PRIORITY);
                tag.write_text_content(BytesText::new(priority))?;
            }

            // TODO extension tags

            Ok(())
        })?;

        Ok(())
    }

    fn finalize(mut self) -> Result<W, Self::Error> {
        let tag = BytesEnd::new(Self::URL_SET);
        self.writer.write_event(Event::End(tag))?;

        Ok(self.writer.into_inner())
    }
}

impl<W: Write> IndexBuilder<W> for XmlBuilder<W> {
    type Error = XmlBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        let mut writer = Writer::new(writer);
        writer.write_bom()?;

        let tag = BytesStart::new(Self::SITEMAP_INDEX);
        let tag = tag.with_attributes(Self::XMLNS);
        writer.write_event(Event::Start(tag))?;

        Ok(Self { writer })
    }

    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error> {
        let tag = self.writer.create_element(Self::SITEMAP);
        tag.write_inner_content(|writer| {
            {
                let location = record.location.to_string();
                let location = location.as_str();
                let tag = writer.create_element(Self::LOCATION);
                tag.write_text_content(BytesText::new(location))?;
            }

            if let Some(last_modified) = &record.last_modified {
                let last_modified = last_modified.to_string();
                let last_modified = last_modified.as_str();
                let tag = writer.create_element(Self::LAST_MODIFIED);
                tag.write_text_content(BytesText::new(last_modified))?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn finalize(mut self) -> Result<W, Self::Error> {
        let tag = BytesEnd::new(Self::SITEMAP_INDEX);
        self.writer.write_event(Event::End(tag))?;

        Ok(self.writer.into_inner())
    }
}
