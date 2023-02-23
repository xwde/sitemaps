use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, Write};
use std::marker::PhantomData;

use crate::attribute::Attribute;
use crate::{Builder, Record, SitemapRecord, BYTES_LIMIT, RECORDS_LIMIT};

// TODO derive PartialEq, Clone
#[derive(Debug)]
pub enum TxtBuilderError {
    TooManyRecords,
    TooManyBytes(usize),
    IoError(IoError),
}

impl Display for TxtBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::TooManyRecords => write!(f, "too many records"),
            Self::TooManyBytes(n) => write!(f, "too many bytes: {n} over limit"),
            Self::IoError(e) => Display::fmt(&e, f),
        }
    }
}

impl From<IoError> for TxtBuilderError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Error for TxtBuilderError {}

/// TXT format sitemap builder.
///
/// ```rust
/// # use sitemaps::{Record, SitemapRecord};
/// # use sitemaps::{Builder, TxtBuilder};
/// # use sitemaps::attribute::{Attribute, Location};
/// let mut buffer = Vec::new();
///
/// // Replace TxtBuilder with XmlBuilder for Xml Sitemap.
/// let mut builder = TxtBuilder::initialize(&mut buffer).unwrap();
///
/// // Replace SitemapRecord with IndexRecord for Sitemap Index.
/// let record = "https://example.com/";
/// let record = Location::parse(record).unwrap();
/// let record = SitemapRecord::new(record);
///
/// builder.next(record).unwrap();
/// builder.finalize().unwrap();
///
/// let sitemap = String::from_utf8_lossy(buffer.as_slice());
/// let sitemap = sitemap.to_string();
/// ```
pub struct TxtBuilder<W: Write, D: Record> {
    record: PhantomData<D>,
    written_bytes: usize,
    written_records: usize,
    writer: W,
}

impl<W: Write, D: Record> TxtBuilder<W, D> {
    const NEWLINE: &'static [u8] = "\n".as_bytes();

    fn new(writer: W) -> Result<Self, TxtBuilderError> {
        Ok(Self {
            record: PhantomData,
            written_bytes: 0,
            written_records: 0,
            writer,
        })
    }
}

impl<W: Write> Builder<W, SitemapRecord> for TxtBuilder<W, SitemapRecord> {
    type Error = TxtBuilderError;

    fn initialize(writer: W) -> Result<Self, Self::Error> {
        Self::new(writer)
    }

    fn next(&mut self, record: SitemapRecord) -> Result<(), Self::Error> {
        if self.written_records + 1 > RECORDS_LIMIT {
            return Err(TxtBuilderError::TooManyRecords);
        }

        let record = record.location.build();
        let record = record.as_str();

        let expected_bytes = record.len() + Self::NEWLINE.len();
        let total_bytes = self.written_bytes + expected_bytes;
        if total_bytes > BYTES_LIMIT {
            let over_limit = total_bytes - BYTES_LIMIT;
            return Err(TxtBuilderError::TooManyBytes(over_limit));
        }

        self.written_bytes += self.writer.write(record.as_bytes())?;
        self.written_records += 1;

        Ok(())
    }

    fn finalize(self) -> Result<W, Self::Error> {
        Ok(self.writer)
    }
}
