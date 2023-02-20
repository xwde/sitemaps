use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;

use crate::parse::SitemapParser;
use crate::SitemapRecord;

#[derive(Debug)]
pub struct TxtParserError {}

impl Display for TxtParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for TxtParserError {}

pub struct TxtParser<R: Read> {
    reader: R,
}

impl<R: Read> SitemapParser<R> for TxtParser<R> {
    type Error = TxtParserError;

    fn create(reader: R) -> Result<Self, Self::Error> {
        Ok(Self { reader })
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }
}
