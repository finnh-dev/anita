#[test]
fn test_macro_expansion() {
    macrotest::expand("tests/expand/*.rs");
}

internal_macros::link_cranelift! {
    fn min(x: f32, y: f32) -> f32 {
        x.min(y)
    }

    fn test_fn(x: f32, y: f32) -> f32 {
        x + y
    }
}
