use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, Write};
use std::marker::PhantomData;

use crate::build::{Builder, BuilderStat};
use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT};
use crate::{Record, SitemapRecord};

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

///
///
/// ```rust
/// use sitemaps::attribute::{Attribute, Location};
/// use sitemaps::build::{Builder, TxtBuilder};
/// use sitemaps::{Record, SitemapRecord};
///
/// // Replace TxtBuilder with XmlBuilder for Txt Sitemap.
/// let mut builder = TxtBuilder::new(Vec::new()).unwrap();
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

impl<W: Write, D: Record> BuilderStat for TxtBuilder<W, D> {
    fn written_bytes(&self) -> usize {
        self.written_bytes
    }

    fn written_records(&self) -> usize {
        self.written_records
    }
}

impl<W: Write> Builder<W, SitemapRecord> for TxtBuilder<W, SitemapRecord> {
    type Error = TxtBuilderError;

    fn new(writer: W) -> Result<Self, Self::Error> {
        Self::new(writer)
    }

    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        if self.written_records + 1 > RECORDS_LIMIT {
            return Err(TxtBuilderError::TooManyRecords);
        }

        let record = record.location.to_string();
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
