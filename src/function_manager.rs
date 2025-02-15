use crate::jit::types::AnitaType;
use internal_macros::function_manager;

pub trait FunctionManager {
    fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]>;
    fn function_signature(
        identifier: &str,
        calling_conventrion: cranelift::prelude::isa::CallConv,
    ) -> Option<cranelift::prelude::Signature>;
}

pub struct DefaultFunctionManager;

#[cfg(feature = "no-default-functions")]
#[function_manager]
impl DefaultFunctionManager {}

#[cfg(not(feature = "no-default-functions"))]
#[function_manager]
impl DefaultFunctionManager {
    #[name = "min"]
    fn internal_min(x: f32, y: f32) -> f32 {
        x.min(y)
    }

    #[name = "max"]
    fn internal_max(x: f32, y: f32) -> f32 {
        x.max(y)
    }

    #[name = "floor"]
    fn internal_floor(x: f32) -> f32 {
        x.floor()
    }

    #[name = "round"]
    fn internal_round(x: f32) -> f32 {
        x.round()
    }

    #[name = "ceil"]
    fn internal_ceil(x: f32) -> f32 {
        x.ceil()
    }

    #[name = "is_nan"]
    fn internal_is_nan(x: f32) -> f32 {
        x.is_nan() as u8 as f32
    }

    #[name = "is_finite"]
    fn internal_is_finite(x: f32) -> f32 {
        x.is_finite() as u8 as f32
    }

    #[name = "is_infinite"]
    fn internal_is_infinite(x: f32) -> f32 {
        x.is_infinite() as u8 as f32
    }

    #[name = "is_normal"]
    fn internal_is_normal(x: f32) -> f32 {
        x.is_normal() as u8 as f32
    }

    #[name = "pow"]
    fn internal_pow(a: f32, x: f32) -> f32 {
        a.powf(x)
    }

    #[name = "mod"]
    fn internal_mod(x: f32, y: f32) -> f32 {
        x % y
    }

    #[name = "ln"]
    fn internal_ln(x: f32) -> f32 {
        x.ln()
    }

    #[name = "log2"]
    fn internal_log2(x: f32) -> f32 {
        x.log2()
    }

    #[name = "log10"]
    fn internal_log10(x: f32) -> f32 {
        x.log10()
    }

    #[name = "exp"]
    fn internal_exp(x: f32) -> f32 {
        x.exp()
    }

    #[name = "exp2"]
    fn internal_exp2(x: f32) -> f32 {
        x.exp2()
    }

    #[name = "cos"]
    fn internal_cos(x: f32) -> f32 {
        x.cos()
    }

    #[name = "acos"]
    fn internal_acos(x: f32) -> f32 {
        x.acos()
    }

    #[name = "cosh"]
    fn internal_cosh(x: f32) -> f32 {
        x.cosh()
    }

    #[name = "acosh"]
    fn internal_acosh(x: f32) -> f32 {
        x.acosh()
    }

    #[name = "sin"]
    fn internal_sin(x: f32) -> f32 {
        x.sin()
    }

    #[name = "asin"]
    fn internal_asin(x: f32) -> f32 {
        x.asin()
    }

    #[name = "sinh"]
    fn internal_sinh(x: f32) -> f32 {
        x.sinh()
    }

    #[name = "asinh"]
    fn internal_asinh(x: f32) -> f32 {
        x.asinh()
    }

    #[name = "tan"]
    fn internal_tan(x: f32) -> f32 {
        x.tan()
    }

    #[name = "atan"]
    fn internal_atan(x: f32) -> f32 {
        x.atan()
    }

    #[name = "atan2"]
    fn internal_atan2(x: f32, y: f32) -> f32 {
        x.atan2(y)
    }

    #[name = "tanh"]
    fn internal_tanh(x: f32) -> f32 {
        x.tanh()
    }

    #[name = "atanh"]
    fn internal_atanh(x: f32) -> f32 {
        x.atanh()
    }

    #[name = "sqrt"]
    fn internal_sqrt(x: f32) -> f32 {
        x.sqrt()
    }

    #[name = "cbrt"]
    fn internal_cbrt(x: f32) -> f32 {
        x.cbrt()
    }

    #[name = "abs"]
    fn internal_abs(x: f32) -> f32 {
        x.abs()
    }

    #[name = "hypot"]
    fn internal_hypot(x: f32, y: f32) -> f32 {
        x.hypot(y)
    }

    #[name = "if"]
    fn internal_if(cond: f32, a: f32, b: f32) -> f32 {
        if cond.is_normal() && cond != 0.0 {
            a
        } else {
            b
        }
    }
}
