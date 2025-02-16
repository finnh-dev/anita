#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg(feature = "simd")]
use anita::compile_expression;
use std::{assert_eq, print, simd::f32x4};

#[test]
fn it_works() {
    let (cf, function) = {
        use anita_core::function_manager::NoFunctions;
        use anita_core::jit::JIT;
        use std::mem;
        let mut jit = JIT::<f32x4, NoFunctions>::default();
        match jit.compile("x*x", &[stringify!(x)]) {
            Ok(code_ptr) => {
                let function_pointer = unsafe {
                    mem::transmute::<*const u8, extern "C" fn(f32x4) -> f32x4>(
                        code_ptr,
                    )
                };
                let memory_region = jit.dissolve();
                Ok((memory_region, function_pointer))
            }
            Err(e) => Err(e),
        }
    }.expect("compilation failed");
    let x = f32x4::splat(5.0);
    let result = function(x);
    println!("result: {:?}\nexpected: {:?}", result, x);
    assert_eq!(x, result);
    // drop(cf);
    panic!("panic for diagnostics");
}
