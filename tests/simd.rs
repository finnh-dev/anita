#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg(feature = "simd")]
use anita::compile_expression;
use std::{assert_eq, println, simd::f32x4};

#[repr(align(16))]
struct AlignedF32x4(f32x4);

#[test]
fn it_works() {
    let function = {
        use anita_core::function_manager::NoFunctions;
        use anita_core::jit::{compiled_function::CompiledFunction, JIT};
        use std::mem;
        let mut jit = JIT::<f32x4, NoFunctions>::default();
        match jit.compile("x", &[stringify!(x)]) {
            Ok(code_ptr) => {
                let function_pointer = unsafe {
                    mem::transmute::<*const u8, fn(compile_expression!(@to_type x,f32x4)) -> f32x4>(
                        code_ptr,
                    )
                };
                let memory_region = jit.dissolve();
                Ok(CompiledFunction::new(memory_region, function_pointer))
            }
            Err(e) => Err(e),
        }
    };
    let x = f32x4::splat(5.0);
    let result = function(x);
    assert_eq!(x, result);
    // panic!("panic for diagnostics");
}
