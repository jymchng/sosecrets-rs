#![feature(generic_const_exprs)]

mod macros;
mod secret;

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::secret::*;
}
