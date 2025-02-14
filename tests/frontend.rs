use anita::compile_expression;

#[test]
fn pow_precedence() {
    let expression = "8 * x ^ 2";
    let function = compile_expression!(expression, (x) -> f32).expect("Compilation failed");
    let result = function(3.0);
    let expected = 8.0 * 3_f32.powf(2.0);
    assert_eq!(result, expected);
}

#[test]
fn whitespace() {
    let expression = "x  ";
    let _function = compile_expression!(expression, (x) -> f32).expect("Compilation failed");
}