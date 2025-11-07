#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod buffer;
pub mod canvas;
pub mod grapheme;
pub mod render_position;
pub mod renderable;
pub mod tag;
