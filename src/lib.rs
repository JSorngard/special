//! [Special functions][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Special_functions

#[cfg(test)]
extern crate assert;

extern crate libc;

mod beta;
mod error;
mod gamma;
mod math;
mod consts;

pub use beta::Beta;
pub use error::Error;
pub use gamma::Gamma;
