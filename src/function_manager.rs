
use internal_macros::function_manager;

pub trait FunctionManager {
    fn function_address(identifier: &str) -> Option<*const u8>;
    fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]>;
    fn function_signature(
        identifier: &str,
        calling_conventrion: cranelift::prelude::isa::CallConv,
    ) -> Option<cranelift::prelude::Signature>;
}

pub struct DefaultFunctionManager;

#[function_manager]
impl DefaultFunctionManager {
    fn min(x: f32, y: f32) -> f32 {
        x.min(y)
    }

    fn max(x: f32, y: f32) -> f32 {
        x.max(y)
    }

    fn floor(x: f32) -> f32 {
        x.floor()
    }

    fn round(x: f32) -> f32 {
        x.round()
    }

    fn ceil(x: f32) -> f32 {
        x.ceil()
    }

    fn is_nan(x: f32) -> f32 {
        x.is_nan() as u8 as f32
    }

    fn is_finite(x: f32) -> f32 {
        x.is_finite() as u8 as f32
    }

    fn is_infinite(x: f32) -> f32 {
        x.is_infinite() as u8 as f32
    }

    fn is_normal(x: f32) -> f32 {
        x.is_normal() as u8 as f32
    }

    fn pow(a: f32, x: f32) -> f32 {
        a.powf(x)
    }

    #[name = "mod"]
    fn modulo(x: f32, y: f32) -> f32 {
        x % y
    }

    fn ln(x: f32) -> f32 {
        x.ln()
    }

    fn log2(x: f32) -> f32 {
        x.log2()
    }

    fn log10(x: f32) -> f32 {
        x.log10()
    }

    fn exp(x: f32) -> f32 {
        x.exp()
    }

    fn exp2(x: f32) -> f32 {
        x.exp2()
    }

    fn cos(x: f32) -> f32 {
        x.cos()
    }

    fn acos(x: f32) -> f32 {
        x.acos()
    }

    fn cosh(x: f32) -> f32 {
        x.cosh()
    }

    fn acosh(x: f32) -> f32 {
        x.acosh()
    }

    fn sin(x: f32) -> f32 {
        x.sin()
    }

    fn asin(x: f32) -> f32 {
        x.asin()
    }

    fn sinh(x: f32) -> f32 {
        x.sinh()
    }

    fn asinh(x: f32) -> f32 {
        x.asinh()
    }

    fn tan(x: f32) -> f32 {
        x.tan()
    }

    fn atan(x: f32) -> f32 {
        x.atan()
    }

    fn atan2(x: f32, y: f32) -> f32 {
        x.atan2(y)
    }

    fn tanh(x: f32) -> f32 {
        x.tanh()
    }

    fn atanh(x: f32) -> f32 {
        x.atanh()
    }

    fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }

    fn cbrt(x: f32) -> f32 {
        x.cbrt()
    }

    fn abs(x: f32) -> f32 {
        x.abs()
    }

    fn hypot(x: f32, y: f32) -> f32 {
        x.hypot(y)
    }

    #[name = "if"]
    fn case(cond: f32, a: f32, b: f32) -> f32 {
        if cond.is_normal() && cond != 0.0 {
            a
        } else {
            b
        }
    }
}
