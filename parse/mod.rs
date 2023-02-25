#[cfg(feature = "txt")]
mod txt;
#[cfg(feature = "txt")]
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

use std::error::Error;
use std::io::Read;

use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT};
use crate::Record;

pub trait ParserStat {
    fn read_bytes(&self) -> usize;
    fn read_records(&self) -> usize;

    fn bytes_until_limit(&self) -> usize {
        BYTES_LIMIT - self.read_bytes()
    }

    fn records_until_limit(&self) -> usize {
        RECORDS_LIMIT - self.read_records()
    }

    fn is_ok(&self) -> bool {
        self.bytes_until_limit() > 0 && self.records_until_limit() > 0
    }
}

pub trait Parser<R: Read, D: Record>: Sized {
    type Error: Error;

    fn new(reader: R) -> Result<Self, Self::Error>;
    fn next(&mut self) -> Result<Option<D>, Self::Error>;
    fn finalize(self) -> Result<R, Self::Error>;
}
