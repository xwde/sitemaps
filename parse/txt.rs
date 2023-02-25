use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Error as IoError, Read, Take};
use std::marker::PhantomData;

use crate::attribute::{Attribute, Location};
use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT};
use crate::parse::{Parser, ParserStat};
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
            Self::IncorrectFormat => write!(f, "beep boop"),
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
/// # use sitemaps::parse::{Parser, TxtParser};
///
/// // Pretend it's our reader.
/// let mut buffer = "https://example.com/".as_bytes();
///
/// // Replace TxtParser with XmlParser for Xml Sitemap.
/// let mut parser = TxtParser::new(&mut buffer).unwrap();
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

    fn try_next(&mut self) -> Result<Option<Location>, TxtParserError> {
        if self.read_records + 1 > RECORDS_LIMIT {
            return Err(TxtParserError::TooManyRecords);
        }

        if self.read_bytes > BYTES_LIMIT {
            let over_limit = self.read_bytes - BYTES_LIMIT;
            return Err(TxtParserError::TooManyBytes(over_limit));
        }

        let mut line = String::new();
        let read_bytes_now = self.reader.read_line(&mut line)?;
        if read_bytes_now == 0 {
            return Ok(None);
        }

        self.read_bytes += read_bytes_now;
        self.read_records += 1;

        if self.read_bytes > BYTES_LIMIT {
            let over_limit = self.read_bytes - BYTES_LIMIT;
            return Err(TxtParserError::TooManyBytes(over_limit));
        }

        let location = Location::parse(line.as_str());
        let location = location.map_err(|_| TxtParserError::IncorrectFormat)?;

        Ok(Some(location))
    }
}

impl<R: Read, D: Record> ParserStat for TxtParser<R, D> {
    fn read_bytes(&self) -> usize {
        self.read_bytes
    }

    fn read_records(&self) -> usize {
        self.read_records
    }
}

impl<R: Read> Parser<R, SitemapRecord> for TxtParser<R, SitemapRecord> {
    type Error = TxtParserError;

    fn new(reader: R) -> Result<Self, Self::Error> {
        Self::new(reader)
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        loop {
            return match self.try_next() {
                Err(TxtParserError::IncorrectFormat) => continue,
                x => Ok(x?.map(|loc| SitemapRecord::new(loc))),
            };
        }
    }

    fn finalize(self) -> Result<R, Self::Error> {
        Ok(self.reader.into_inner().into_inner())
    }
}
