use core::f32;

use anita::compile_expression;

const MIN: f32 = f32::MIN_POSITIVE; // 1.17549435e-38f32
const MAX: f32 = f32::MAX;
const LOWER_THAN_MIN: f32 = 1.0e-40_f32;
const ZERO: f32 = 0.0_f32;

const TEST_VALUES: [(&str, f32); 9] = [
    ("MAX", f32::MAX),
    ("MIN", f32::MIN),
    ("EPSILON", f32::EPSILON),
    ("MIN_POSITIVE", f32::MIN_POSITIVE),
    ("NAN", f32::NAN),
    ("INFINITY", f32::INFINITY),
    ("NEG_INFINITY", f32::NEG_INFINITY),
    ("PI", f32::consts::PI),
    ("E", f32::consts::E),
];


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
fn case() {
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

fn test_unspecified_precision_function(expression: &str, function: fn(f32) -> f32) {
    let func = compile_expression!(expression, (x) -> f32).unwrap();
    for (c, test_value) in TEST_VALUES {
        let result = func.execute(test_value);
        let expected = function(test_value);
        if test_eq(result, expected) {
            println!("{c}: {result} == {expected}");
        } else {
            let difference = (result - expected).abs();
            println!("{c}: {result} - {expected} = {difference}");
            assert!(difference < f32::EPSILON);
        }
    }
}

fn test_eq(a: f32, b: f32) -> bool {
    (a == b)
        || (a.is_nan() && b.is_nan())
        || if a.is_infinite() && b.is_infinite() {
            a.signum() == b.signum()
        } else {
            false
        }
}

#[test]
fn ln() {
    test_unspecified_precision_function("ln(x)", f32::ln);
}

#[test]
fn log2() {
    test_unspecified_precision_function("log2(x)", f32::log2);
}

#[test]
fn log10() {
    test_unspecified_precision_function("log10(x)", f32::log10);
}

#[test]
fn exp() {
    test_unspecified_precision_function("exp(x)", f32::exp);
}

#[test]
fn exp2() {
    test_unspecified_precision_function("exp2(x)", f32::exp2);
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
fn hypot() {
    let func = compile_expression!("hypot(x, 2.5)", (x) -> f32).unwrap();
    let result = func.execute(2.0);
    assert_eq!(result, f32::hypot(2.0, 2.5));
}

#[test]
fn cos() {
    test_unspecified_precision_function("cos(x)", f32::cos);
}

#[test]
fn acos() {
    test_unspecified_precision_function("acos(x)", f32::acos);
}

#[test]
fn cosh() {
    test_unspecified_precision_function("cosh(x)", f32::cosh);
}

#[test]
fn acosh() {
    test_unspecified_precision_function("acosh(x)", f32::acosh);
}

#[test]
fn sin() {
    test_unspecified_precision_function("sin(x)", f32::sin);
}

#[test]
fn asin() {
    test_unspecified_precision_function("asin(x)", f32::asin);
}

#[test]
fn sinh() {
    test_unspecified_precision_function("sinh(x)", f32::sinh);
}

#[test]
fn asinh() {
    test_unspecified_precision_function("asinh(x)", f32::asinh);
}

#[test]
fn tan() {
    test_unspecified_precision_function("tan(x)", f32::tan);
}

#[test]
fn atan() {
    test_unspecified_precision_function("atan(x)", f32::atan);
}

#[test]
fn tanh() {
    test_unspecified_precision_function("tanh(x)", f32::tanh);
}

#[test]
fn atanh() {
    test_unspecified_precision_function("atanh(x)", f32::atanh);
}

#[test]
fn sqrt() {
    test_unspecified_precision_function("sqrt(x)", f32::sqrt);
}

#[test]
fn cbrt() {
    test_unspecified_precision_function("cbrt(x)", f32::cbrt);
}

#[test]
fn abs() {
    let func = compile_expression!("abs(x)", (x) -> f32).unwrap();
    let result = func.execute(1.0);
    assert_eq!(result, f32::abs(1.0));
    let result = func.execute(-1.0);
    assert_eq!(result, f32::abs(-1.0));
}
