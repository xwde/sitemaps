use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Error as IoError, Read, Take};
use std::marker::PhantomData;

use crate::attribute::{Attribute, Location};
use crate::{Parser, Record, SitemapRecord, BYTES_LIMIT, RECORDS_LIMIT};

#[derive(Debug)]
pub enum TxtParserError {
    TooManyRecords,
    TooManyBytes(usize),
    IoError(IoError),
    IncorrectFormat,
}

impl Display for TxtParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::TooManyRecords => write!(f, "too many records"),
            Self::TooManyBytes(n) => write!(f, "too many bytes: {n} over limit"),
            Self::IoError(e) => Display::fmt(&e, f),
            Self::IncorrectFormat => unreachable!(),
        }
    }
}

impl From<IoError> for TxtParserError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Error for TxtParserError {}

///
///
/// ```rust
/// ```
pub struct TxtParser<R: Read, D: Record> {
    record: PhantomData<D>,
    read_bytes: usize,
    read_records: usize,
    reader: Take<BufReader<R>>,
}

impl<R: Read, D: Record> TxtParser<R, D> {
    const MAX_URL_LENGTH: u64 = 16_384;

    fn new(reader: R) -> Result<Self, TxtParserError> {
        let reader = BufReader::new(reader).take(Self::MAX_URL_LENGTH);
        Ok(Self {
            record: PhantomData,
            read_bytes: 0,
            read_records: 0,
            reader,
        })
    }

    fn try_next(&mut self) -> Result<Option<D>, TxtParserError> {
        if self.read_records + 1 > RECORDS_LIMIT {
            return Err(TxtParserError::TooManyRecords);
        }

        if self.read_bytes > BYTES_LIMIT {
            let over_limit = self.read_bytes - BYTES_LIMIT;
            return Err(TxtParserError::TooManyBytes(over_limit));
        }

        let mut line = String::new();
        self.read_bytes += self.reader.read_line(&mut line)?;
        self.read_records += 1;

        if self.read_bytes > BYTES_LIMIT {
            let over_limit = self.read_bytes - BYTES_LIMIT;
            return Err(TxtParserError::TooManyBytes(over_limit));
        }

        let location = Location::parse(line.as_str());
        let location = location.map_err(|_| TxtParserError::IncorrectFormat)?;
        let record = D::new(location);

        Ok(Some(record))
    }
}

impl<R: Read> Parser<R, SitemapRecord> for TxtParser<R, SitemapRecord> {
    type Error = TxtParserError;

    fn initialize(reader: R) -> Result<Self, Self::Error> {
        Self::new(reader)
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        loop {
            match self.try_next() {
                Err(TxtParserError::IncorrectFormat) => continue,
                x => return x,
            }
        }
    }

    fn finalize(self) -> Result<R, Self::Error> {
        Ok(self.reader.into_inner().into_inner())
    }
}
