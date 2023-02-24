use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Write;
use std::marker::PhantomData;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Error as XmlError, Writer};

use crate::build::Builder;
use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT};
use crate::{IndexRecord, Record, SitemapRecord};

// TODO derive PartialEq
#[derive(Debug, Clone)]
pub enum XmlBuilderError {
    TooManyRecords,
    TooManyBytes(usize),
    XmlError(XmlError),
}

impl Display for XmlBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::TooManyRecords => write!(f, "too many records"),
            Self::TooManyBytes(n) => write!(f, "too many bytes: {n} over limit"),
            Self::XmlError(e) => Display::fmt(&e, f),
        }
    }
}

impl From<XmlError> for XmlBuilderError {
    fn from(error: XmlError) -> Self {
        Self::XmlError(error)
    }
}

impl Error for XmlBuilderError {}

///
///
/// ```rust
/// use sitemaps::attribute::{Attribute, Location};
/// use sitemaps::build::{Builder, XmlBuilder};
/// use sitemaps::{Record, SitemapRecord};
///
/// // Replace XmlBuilder with TxtBuilder for Txt Sitemap.
/// let mut builder = XmlBuilder::initialize(Vec::new()).unwrap();
///
/// // Replace SitemapRecord with IndexRecord for Sitemap Index.
/// let record = "https://example.com/";
/// let record = Location::parse(record).unwrap();
/// let record = SitemapRecord::new(record);
///
/// builder.next(&record).unwrap();
/// let buffer = builder.finalize().unwrap();
///
/// let sitemap = String::from_utf8_lossy(buffer.as_slice());
/// println!("{}", sitemap.to_string());
/// ```
pub struct XmlBuilder<W: Write, D: Record> {
    record: PhantomData<D>,
    written_bytes: usize,
    written_records: usize,
    writer: Writer<W>,
}

impl<W: Write, D: Record> XmlBuilder<W, D> {
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

    fn new(writer: W, seal: &str) -> Result<Self, XmlBuilderError> {
        let mut writer = Writer::new(writer);
        writer.write_bom()?;

        let tag = BytesStart::new(seal);
        let tag = tag.with_attributes(Self::XMLNS);
        writer.write_event(Event::Start(tag))?;

        // TODO extensions attributes
        #[cfg(feature = "extension")]
        {}

        Ok(Self {
            record: PhantomData,
            written_bytes: 0,
            written_records: 0,
            writer,
        })
    }

    fn seal(mut self, seal: &str) -> Result<W, XmlBuilderError> {
        let tag = BytesEnd::new(seal);
        self.writer.write_event(Event::End(tag))?;

        Ok(self.writer.into_inner())
    }
}

impl<W: Write> Builder<W, SitemapRecord> for XmlBuilder<W, SitemapRecord> {
    type Error = XmlBuilderError;

    fn initialize(writer: W) -> Result<Self, Self::Error> {
        Self::new(writer, Self::URL_SET)
    }

    // TODO count bytes
    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        if self.written_records + 1 > RECORDS_LIMIT {
            return Err(Self::Error::TooManyRecords);
        }

        if self.written_bytes > BYTES_LIMIT {
            let over_limit = self.written_bytes - BYTES_LIMIT;
            return Err(Self::Error::TooManyBytes(over_limit));
        }

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
                let change_frequency = change_frequency.to_string();
                let change_frequency = change_frequency.as_str();
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
            #[cfg(feature = "extension")]
            {}

            Ok(())
        })?;

        self.written_records += 1;
        Ok(())
    }

    fn finalize(self) -> Result<W, Self::Error> {
        self.seal(Self::URL_SET)
    }

    fn written_bytes(&self) -> usize {
        self.written_bytes
    }

    fn written_records(&self) -> usize {
        self.written_records
    }
}

impl<W: Write> Builder<W, IndexRecord> for XmlBuilder<W, IndexRecord> {
    type Error = XmlBuilderError;

    fn initialize(writer: W) -> Result<Self, Self::Error> {
        Self::new(writer, Self::SITEMAP_INDEX)
    }

    // TODO count bytes
    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error> {
        if self.written_records + 1 > RECORDS_LIMIT {
            return Err(Self::Error::TooManyRecords);
        }

        if self.written_bytes > BYTES_LIMIT {
            let over_limit = self.written_bytes - BYTES_LIMIT;
            return Err(Self::Error::TooManyBytes(over_limit));
        }

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

        self.written_records += 1;
        Ok(())
    }

    fn finalize(self) -> Result<W, Self::Error> {
        self.seal(Self::SITEMAP_INDEX)
    }

    fn written_bytes(&self) -> usize {
        self.written_bytes
    }

    fn written_records(&self) -> usize {
        self.written_records
    }
}
