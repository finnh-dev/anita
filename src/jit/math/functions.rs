use std::vec;

use cranelift::prelude::{isa::CallConv, types, AbiParam, Signature};

pub(crate) fn get_function_signature(identifier: &str) -> Option<Signature> {
    match identifier {
        "test_fn" => Some(Signature {
            params: vec![AbiParam::new(types::F32), AbiParam::new(types::F32)],
            returns: vec![AbiParam::new(types::F32)],
            call_conv: CallConv::SystemV,
        }),
        "min" => Some(Signature {
            params: vec![AbiParam::new(types::F32), AbiParam::new(types::F32)],
            returns: vec![AbiParam::new(types::F32)],
            call_conv: CallConv::SystemV,
        }),
        _ => None
    }
}

pub(crate) fn get_function_addr(identifier: &str) -> Option<*const u8> {
    match identifier {
        "test_fn" => Some(test_fn as *const u8),
        _ => None
    }
}

#[no_mangle]
pub extern "C" fn test_fn(x: f32, y: f32) -> f32 {
    x + y
}

#[no_mangle]
pub extern "C" fn min(x: f32, y: f32) -> f32 {
    x.min(y)
}