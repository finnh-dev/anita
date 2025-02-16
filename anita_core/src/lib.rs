#![deny(unused_must_use)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::unwrap_used)]
#![cfg_attr(feature = "simd", feature(portable_simd))]

pub mod function_manager;
pub mod jit;

pub use cranelift;
