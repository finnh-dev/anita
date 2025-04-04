struct TestFunctionManager;

#[cfg(not(feature="whatever"))]
#[internal_macros::function_manager]
impl TestFunctionManager {
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
}