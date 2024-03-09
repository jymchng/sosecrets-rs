#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Apply #![recursion_limit = "256"] only if #![cfg(target_pointer_width = "128")]
#![recursion_limit = "2048"]

#[cfg(feature = "alloc")]
extern crate alloc;

// #[cfg(feature = "runtime-secret")]
pub mod runtime;

mod macros;
mod secret;

pub mod traits;
pub mod types;

pub mod prelude {
    pub use crate::{runtime::*, secret::*, types::*};
}
