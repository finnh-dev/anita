internal_macros::link_cranelift! {
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

    fn sin(x: f32) -> f32 {
        x.sin()
    }

    fn case(cond: f32, a: f32, b: f32) -> f32 {
        if cond != 0.0 {
            a
        } else {
            b
        }
    }
}
