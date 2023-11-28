#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod macros;
mod secret;

pub mod traits;

pub mod prelude {
    pub use crate::secret::*;
}
