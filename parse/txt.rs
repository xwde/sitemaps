use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Error as IoError, Read, Take};
use std::marker::PhantomData;

use crate::attribute::{Attribute, Location};
use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT, RESOURCE_LENGTH_LIMIT};
use crate::parse::Parser;
use crate::{Record, SitemapRecord};

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
/// use sitemaps::parse::{Parser, TxtParser};
///
/// // Pretend it's our reader
/// let mut buffer = "https://example.com/".as_bytes();
///
/// // Replace TxtParser with XmlParser for Xml Sitemap.
/// let mut parser = TxtParser::initialize(&mut buffer).unwrap();
/// while let Some(record) = parser.next().ok().flatten() {
///     println!("{}", record.location.to_string());
/// }
///
/// parser.finalize().unwrap();
/// ```
pub struct TxtParser<R: Read, D: Record> {
    record: PhantomData<D>,
    read_bytes: usize,
    read_records: usize,
    reader: Take<BufReader<R>>,
}

impl<R: Read, D: Record> TxtParser<R, D> {
    fn new(reader: R) -> Result<Self, TxtParserError> {
        let reader = BufReader::new(reader).take(RESOURCE_LENGTH_LIMIT as u64);
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

    fn read_bytes(&self) -> usize {
        self.read_bytes
    }

    fn read_records(&self) -> usize {
        self.read_records
    }
}
