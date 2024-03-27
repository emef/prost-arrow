#[allow(unused_imports)]
#[macro_use]
extern crate prost_arrow_derive;

#[doc(hidden)]
pub use prost_arrow_derive::*;

mod binary;
mod lists;
mod primitives;
mod traits;

pub use binary::*;
pub use lists::*;
pub use primitives::*;
pub use traits::*;

pub fn new_builder<T: ToArrow>() -> T::Builder {
    <T as ToArrow>::Builder::new_with_capacity(0)
}

pub fn new_builder_with_capacity<T: ToArrow>(capacity: usize) -> T::Builder {
    <T as ToArrow>::Builder::new_with_capacity(capacity)
}
