#![deny(unused_must_use)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::unwrap_used)]

pub mod function_manager;
pub mod jit;

pub use cranelift;
