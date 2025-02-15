use anita_core::compile_expression;

#[test]
fn exponentiation() {
    let func = compile_expression!("x ^ 2", (x) -> f32).expect("Compilation failed");
    let result = func(2.0);
    assert_eq!(result, 2.0_f32.powf(2.0));
}

#[test]
fn product() {
    let func = compile_expression!("x * 2", (x) -> f32).expect("Compilation failed");
    let result = func(1.5);
    assert_eq!(result, 1.5 * 2.0);
}

#[test]
fn division() {
    let func = compile_expression!("x / 2", (x) -> f32).expect("Compilation failed");
    let result = func(2.0);
    assert_eq!(result, 2.0 / 2.0);
}

#[test]
fn modulo() {
    let func = compile_expression!("x % 2", (x) -> f32).expect("Compilation failed");
    let result = func(3.0);
    assert_eq!(result, 3.0 % 2.0);
}

#[test]
fn sum() {
    let func = compile_expression!("x + 2", (x) -> f32).expect("Compilation failed");
    let result = func(5.0);
    assert_eq!(result, 5.0 + 2.0);
}

#[test]
fn difference() {
    let func = compile_expression!("x - 2", (x) -> f32).expect("Compilation failed");
    let result = func(5.0);
    assert_eq!(result, 3.0);
}

#[test]
fn assignment() {
    let func = compile_expression!("y = 2; x + y", (x) -> f32).expect("Compilation failed");
    let result = func(2.0);
    assert_eq!(result, 2.0 + 2.0);
}

#[test]
fn chain() {
    let func =
        compile_expression!("y = 2; z = 2; x + y + z", (x) -> f32).expect("Compilation failed");
    let result = func(2.0);
    assert_eq!(result, 2.0 + 2.0 + 2.0);
}

#[test]
fn neg() {
    let func = compile_expression!("-x", (x) -> f32).expect("Compilation failed");
    let result = func(2.0);
    assert_eq!(result, -2.0);
}
