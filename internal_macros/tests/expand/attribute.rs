internal_macros::link_cranelift! {
    #[name = "mod"]
    fn modulo(x: f32, y: f32) -> f32 {
        x % y
    }
}