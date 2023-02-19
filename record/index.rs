use crate::attribute::LastModified;
use crate::attribute::{Location, LocationError};

use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct IndexRecord {
    pub location: Location,
    pub last_modified: Option<LastModified>,
}

impl IndexRecord {
    pub fn parse(url: &str) -> Result<Self, LocationError> {
        let url = Url::parse(url)?;
        Ok(Self::new(url))
    }

    pub fn new(url: Url) -> Self {
        Self {
            location: Location::new(url),
            last_modified: None,
        }
    }
}

impl IndexRecord {
    pub fn replace_last_modified(&mut self, last_modified: LastModified) -> &mut Self {
        self.last_modified = Some(last_modified);
        self
    }
}
