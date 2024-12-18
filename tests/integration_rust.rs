use core::f32;

use anita::{compile_expression, jit::EvalexprCompError};
use evalexpr::build_operator_tree;
use internal_macros::function_manager;

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
    match result {
        Err(EvalexprCompError::ExpressionEvaluatesToNoValue(node)) => assert_eq!(node, ast),
        Ok(_) => panic!("Expected Error but got successful compilation"),
        Err(e) => panic!(
            "Expected EvalexprCompError::ExpressionEvaluatesToNoValue but got: {:?}",
            e
        ),
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

struct TestFunctionManager;

#[cfg(not(feature = "no-default-functions"))]
#[function_manager]
impl TestFunctionManager {
    #[name = "tanh"]
    fn custom_tanh(x: f32) -> f32 {
        match x {
            f32::INFINITY => 1.0,
            f32::NEG_INFINITY => -1.0,
            x if x.is_nan() => 0.0,
            x => f32::tanh(x),
        }
    }
}

#[cfg(feature = "no-default-functions")]
#[function_manager]
impl TestFunctionManager {
    fn tanh(x: f32) -> f32 {
        match x {
            f32::INFINITY => 1.0,
            f32::NEG_INFINITY => -1.0,
            x if x.is_nan() => 0.0,
            x => f32::tanh(x),
        }
    }
}

#[test]
fn custom_function_manager() {
    let func = compile_expression!("tanh(x)", (x) -> f32, TestFunctionManager).unwrap();
    let result = func(f32::INFINITY);
    assert_eq!(result, 1.0);
    let result = func(f32::NEG_INFINITY);
    assert_eq!(result, -1.0);
    let result = func(f32::NAN);
    assert_eq!(result, 0.0);
    let result = func(1.0);
    assert_eq!(result, f32::tanh(1.0));
}
