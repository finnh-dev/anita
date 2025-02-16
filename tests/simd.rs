#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg(feature = "simd")]
use anita::compile_expression;
use std::{assert_eq, simd::f32x4};

#[test]
fn it_works() {
    let function = compile_expression!("x * x", (x) -> f32x4).expect("compilation failed");
    let x = f32x4::splat(5.0);
    let result = function(x);
    let expected = x * x;
    assert_eq!(expected, result);
}
