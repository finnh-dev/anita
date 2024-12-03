use core::f32;

use evalexpr::build_operator_tree;
use anita::{compile_expression, jit::EvalexprCompError};

#[test]
fn owned_input() {
    let input = "x".to_owned();
    let f = compile_expression!(input, (x) -> f32).unwrap();
    let result = f.execute(1.0);
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
fn division_by_zero() {
    let func = compile_expression!("1.0 / x", (x) -> f32).unwrap();
    let result = func.execute(0.0);
    assert!(result.is_infinite());
}


#[test]
fn sum_absorbed() {
    let func = compile_expression!("x + 1", (x) -> f32).unwrap();
    let result = func.execute(f32::MAX);
    println!("result: {result}, expected: {}", f32::MAX);
    assert_eq!(result, f32::MAX);
}

#[test]
fn sum_nan() {
    let func = compile_expression!("x + 1", (x) -> f32).unwrap();
    let result = func.execute(f32::NAN);
    println!("result: {result}, expected: {}", f32::NAN);
    assert!(result.is_nan())
}