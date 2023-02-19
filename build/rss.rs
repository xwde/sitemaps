use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Write;

use quick_xml::Writer;

use crate::build::SitemapBuilder;
use crate::SitemapRecord;

#[derive(Debug)]
pub enum RssBuilderError {}

impl Display for RssBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for RssBuilderError {}

pub struct RssBuilder<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> SitemapBuilder<W> for RssBuilder<W> {
    type Error = RssBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        todo!()
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
