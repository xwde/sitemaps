#[cfg(feature = "xml")]
mod image;
#[cfg(feature = "xml")]
mod news;
#[cfg(feature = "xml")]
mod video;

#[cfg(feature = "xml")]
pub use image::*;
#[cfg(feature = "xml")]
pub use news::*;
#[cfg(feature = "xml")]
pub use video::*;
