#![forbid(unsafe_code)]
#![feature(type_alias_impl_trait)]
#![feature(return_position_impl_trait_in_trait)]

pub mod attribute;
pub mod converter;
pub mod record;

pub use converter::*;
