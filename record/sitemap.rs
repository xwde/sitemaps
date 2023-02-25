use crate::attribute::ChangeFrequency;
use crate::attribute::LastModified;
use crate::attribute::Location;
use crate::attribute::Priority;
use crate::Record;

#[derive(Debug, Clone, PartialEq)]
pub struct SitemapRecord {
    pub location: Location,
    #[cfg(feature = "xml")]
    pub last_modified: Option<LastModified>,
    #[cfg(feature = "xml")]
    pub change_frequency: Option<ChangeFrequency>,
    #[cfg(feature = "xml")]
    pub priority: Option<Priority>,
}

impl SitemapRecord {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            #[cfg(feature = "xml")]
            last_modified: None,
            #[cfg(feature = "xml")]
            change_frequency: None,
            #[cfg(feature = "xml")]
            priority: None,
        }
    }
}

#[cfg(feature = "xml")]
impl SitemapRecord {
    pub fn with_timestamp(&mut self, modified: LastModified) -> &mut Self {
        self.last_modified = Some(modified);
        self
    }

    pub fn with_frequency(&mut self, frequency: ChangeFrequency) -> &mut Self {
        self.change_frequency = Some(frequency);
        self
    }

    pub fn with_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = Some(priority);
        self
    }
}

impl Record for SitemapRecord {}

impl From<Location> for SitemapRecord {
    fn from(location: Location) -> Self {
        Self::new(location)
    }
}
