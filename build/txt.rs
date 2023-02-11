use crate::attribute::AsUnderlying;
use crate::build::{SitemapBuilder, SitemapState};
use crate::record::{SitemapRecord, BYTES_LIMIT, RECORDS_LIMIT};

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, Write};

#[derive(Debug)]
pub enum TxtBuilderError {
    TooManyRecords(usize),
    TooManyBytes(usize),
    IoError(IoError),
}

impl Display for TxtBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl From<IoError> for TxtBuilderError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Error for TxtBuilderError {}

#[derive(Debug)]
pub struct TxtSitemapState<W: Write> {
    bytes: usize,
    records: usize,
    writer: W,
}

impl<W: Write> TxtSitemapState<W> {
    const NEWLINE: &'static [u8] = "\n".as_bytes();
}

impl<W: Write> SitemapState<W> for TxtSitemapState<W> {
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

    fn finalize(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TxtSitemapBuilder {}

impl<W: Write> SitemapBuilder<W> for TxtSitemapBuilder {
    type State = TxtSitemapState<W>;
}
