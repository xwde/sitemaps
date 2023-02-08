use crate::attribute::location::Location;
use crate::attribute::modified::LastModified;

#[derive(Debug, PartialEq, Clone)]
pub struct SitemapIndexEntry {
    pub location: Location,
    pub last_modified: Option<LastModified>,
}

impl SitemapIndexEntry {
    pub fn parse() -> Self {
        todo!()
    }

    pub fn new() -> Self {
        todo!()
    }
}
