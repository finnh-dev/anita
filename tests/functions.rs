use core::f32;

use anita::compile_expression;

const MIN: f32 = f32::MIN_POSITIVE; // 1.17549435e-38f32
const MAX: f32 = f32::MAX;
const LOWER_THAN_MIN: f32 = 1.0e-40_f32;
const ZERO: f32 = 0.0_f32;

#[test]
fn min() {
    let func = compile_expression!("min(x, 1)", (x) -> f32).unwrap();
    let result = func.execute(2.0);
    assert_eq!(result, f32::min(2.0, 1.0));
    let result = func.execute(0.0);
    assert_eq!(result, f32::min(0.0, 1.0));
}

#[test]
fn max() {
    let func = compile_expression!("max(x, 1)", (x) -> f32).unwrap();
    let result = func.execute(2.0);
    assert_eq!(result, f32::max(2.0, 1.0));
    let result = func.execute(0.0);
    assert_eq!(result, f32::max(0.0, 1.0));
}

#[test]
fn floor() {
    let func = compile_expression!("floor(x)", (x) -> f32).unwrap();
    let result = func.execute(2.5);
    assert_eq!(result, f32::floor(2.5));
}

#[test]
fn round() {
    let func = compile_expression!("round(x)", (x) -> f32).unwrap();
    let result = func.execute(2.7);
    assert_eq!(result, f32::round(2.7));
    let result = func.execute(2.5);
    assert_eq!(result, f32::round(2.5));
    let result = func.execute(2.2);
    assert_eq!(result, f32::round(2.2));
}

#[test]
fn ceil() {
    let func = compile_expression!("ceil(x)", (x) -> f32).unwrap();
    let result = func.execute(2.5);
    assert_eq!(result, f32::ceil(2.5));
}

#[test]
#[ignore = "test not yet implemented"]
fn if_function() {
    let func = compile_expression!("case(is_normal(x), x, 0.0)", (x) -> f32).unwrap();
    let result = func.execute(2.5);
    assert_eq!(result, 2.5);
    let result = func.execute(f32::INFINITY);
    assert_eq!(result, 0.0);
}

#[test]
fn is_nan() {
    let func = compile_expression!("is_nan(x)", (x) -> f32).unwrap();
    let result = func.execute(f32::NAN);
    assert!(result == 1.0);
    let result = func.execute(1.0);
    assert!(result == 0.0);
}

#[test]
fn is_finite() {
    let func = compile_expression!("is_finite(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert!(result == 1.0);
    let result = func.execute(f32::INFINITY);
    assert!(result == 0.0);
    let result = func.execute(f32::NAN);
    assert!(result == 0.0);
}

#[test]
fn is_infinite() {
    let func = compile_expression!("is_infinite(x)", (x) -> f32).unwrap();
    let result = func.execute(f32::INFINITY);
    assert!(result == 1.0);
    let result = func.execute(f32::NEG_INFINITY);
    assert!(result == 1.0);
    let result = func.execute(f32::NAN);
    assert!(result == 0.0);
    let result = func.execute(1.0);
    assert!(result == 0.0);
    let _ = 1_f32.is_infinite();
}

#[test]
fn is_normal() {
    let func = compile_expression!("is_normal(x)", (x) -> f32).unwrap();
    let result = func.execute(MIN);
    assert!(result == 1.0);
    let result = func.execute(MAX);
    assert!(result == 1.0);
    let result = func.execute(ZERO);
    assert!(result == 0.0);
    let result = func.execute(LOWER_THAN_MIN);
    assert!(result == 0.0);
    let result = func.execute(f32::INFINITY);
    assert!(result == 0.0);
    let result = func.execute(f32::NAN);
    assert!(result == 0.0);
}

#[test]
#[ignore = "test process is not yet sound"]
fn ln() {
    let func = compile_expression!("ln(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::ln(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn log2() {
    let func = compile_expression!("log2(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::log2(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn log10() {
    let func = compile_expression!("log10(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::log10(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn exp() {
    let func = compile_expression!("exp(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::exp(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn exp2() {
    let func = compile_expression!("exp2(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::exp2(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn pow() {
    let func = compile_expression!("pow(x, 2.5)", (x) -> f32).unwrap();
    let result = func.execute(2.0);
    assert_eq!(result, f32::powf(2.0, 2.5)); 
}

#[test]
#[ignore = "test process is not yet sound"]
fn cos() {
    let func = compile_expression!("cos(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::cos(1.0)); // TODO: fix non deterministic precision
}

#[test]
#[ignore = "test process is not yet sound"]
fn sin() {
    let func = compile_expression!("sin(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::sin(1.0)); // TODO: fix non deterministic precision
}

