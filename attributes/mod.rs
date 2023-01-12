pub mod frequency;
pub mod location;
pub mod modification;
pub mod priority;

pub trait AsAttribute<T> {
    fn as_attribute(&self) -> T;
}
