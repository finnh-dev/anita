#[test]
fn test_macro_expansion() {
    macrotest::expand("tests/expand/*test.rs");
}

// struct TestFunctionManager;

// #[internal_macros::function_manager]
// impl TestFunctionManager {
//     fn modulo(x: f32, y: f32) -> f32 {
//         x % y
//     }
// }
