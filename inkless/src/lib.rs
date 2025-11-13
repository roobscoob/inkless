#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use inkless_core::*;
pub use inkless_macros::*;

pub mod term {
    pub use inkless_term::*;
}

pub mod widgets {
    pub use inkless_widgets::*;
}
