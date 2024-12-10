use core::f32;

use anita::{compile_expression, jit::EvalexprCompError};
use evalexpr::build_operator_tree;

#[test]
fn owned_input() {
    let input = "x".to_owned();
    let f = compile_expression!(input, (x) -> f32).unwrap();
    let result = f(1.0);
    assert_eq!(result, 1_f32);
}

#[test]
fn empty_input() {
    let expr = "";
    let ast = build_operator_tree(&expr).unwrap();
    let result = compile_expression!(expr, (x) -> f32);
    if let Err(EvalexprCompError::ExpressionEvaluatesToNoValue(node)) = result {
        assert_eq!(node, ast);
    } else {
        panic!(
            "Expected EvalexprCompError::ExpressionEvaluatesToNoValue but got: {:?}",
            result
        )
    }
}

#[test]
fn complex_function() {
    let expression = "tanh(a * x^3) + b * sin(c * x)";
    let function = compile_expression!(expression, (x, a, b, c) -> f32).unwrap();
    let result = function(3.0, 0.7, 0.1, 6.4);
    let expected = f32::tanh(0.7 * f32::powf(3.0, 3.0)) + 0.1 * f32::sin(6.4 * 3.0);
    assert_eq!(result, expected);
}
