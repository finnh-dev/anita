struct TestFunctionManager;

#[internal_macros::function_manager]
impl TestFunctionManager {
    #[name = "mod"]
    fn modulo(x: f32, y: f32) -> f32 {
        x % y
    }
}