pub use frequency::*;
pub use last_mod::*;
pub use location::*;
pub use priority::*;

mod frequency;
mod last_mod;
mod location;
mod priority;

pub trait AsAttribute<T> {
    fn as_attribute(&self) -> T;
}
