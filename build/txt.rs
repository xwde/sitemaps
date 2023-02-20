use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, Write};

use crate::attribute::AsUnderlying;
use crate::build::SitemapBuilder;
use crate::{SitemapRecord, BYTES_LIMIT, RECORDS_LIMIT};

#[derive(Debug)]
pub enum TxtBuilderError {
    TooManyRecords(usize),
    TooManyBytes(usize),
    IoError(IoError),
}

impl Display for TxtBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::TooManyRecords(e) => todo!(),
            Self::TooManyBytes(e) => todo!(),
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
/// # use sitemaps::build::{SitemapBuilderString, TxtBuilder};
/// # use sitemaps::SitemapRecord;
/// let uri = "https://www.example.com/";
/// let record = SitemapRecord::parse(uri).unwrap();
/// let records = vec![record /* & more records... */];
/// let sitemap = TxtBuilder::build_string(records.iter()).unwrap();
/// ```
#[derive(Debug)]
pub struct TxtBuilder<W: Write> {
    bytes: usize,
    records: usize,
    writer: W,
}

impl<W: Write> TxtBuilder<W> {
    const NEWLINE: &'static [u8] = "\n".as_bytes();
}

impl<W: Write> SitemapBuilder<W> for TxtBuilder<W> {
    type Error = TxtBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        Ok(Self {
            bytes: 0,
            records: 0,
            writer,
        })
    }

    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        if self.records + 1 > RECORDS_LIMIT {
            return Err(TxtBuilderError::TooManyRecords(self.records + 1));
        }

        let record = record.location.as_underlying();
        let record = record.as_str();

        let len = self.bytes + record.len() + Self::NEWLINE.len();
        if len > BYTES_LIMIT {
            return Err(TxtBuilderError::TooManyBytes(len));
        }

        if self.records != 0 {
            self.bytes += self.writer.write(Self::NEWLINE)?;
        }

        self.records += 1;
        self.bytes += self.writer.write(record.as_bytes())?;

        Ok(())
    }

    fn finalize(mut self) -> Result<W, Self::Error> {
        Ok(self.writer)
    }
}
