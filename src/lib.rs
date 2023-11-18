#![feature(generic_const_exprs)]

mod secret;
mod macros;

pub mod prelude {
    pub use crate::secret::*;
    pub use crate::macros::*;
}
