use core::f32;

use anita::compile_expression;

const MIN: f32 = f32::MIN_POSITIVE; // 1.17549435e-38f32
const MAX: f32 = f32::MAX;
const LOWER_THAN_MIN: f32 = 1.0e-40_f32;
const ZERO: f32 = 0.0_f32;

const TEST_VALUES: [(&str, f32); 17] = [
    ("MAX", f32::MAX),
    ("MIN", f32::MIN),
    ("EPSILON", f32::EPSILON),
    ("MIN_POSITIVE", f32::MIN_POSITIVE),
    ("NAN", f32::NAN),
    ("INFINITY", f32::INFINITY),
    ("NEG_INFINITY", f32::NEG_INFINITY),
    ("PI", f32::consts::PI),
    ("E", f32::consts::E),
    ("ONE", 1.0),
    ("TWO", 2.0),
    ("ZERO", 0.0),
    ("NEG_ONE", -1.0),
    ("NEG_TWO", -2.0),
    ("2.5", 2.5),
    ("2.7", 2.7),
    ("2.2", 2.2),
];

fn test_function(expression: &str, function: fn(f32) -> f32) {
    let func = compile_expression!(expression, (x) -> f32).unwrap();
    for (c, test_value) in TEST_VALUES {
        let result = func(test_value);
        let expected = function(test_value);
        println!("{c}: {result} == {expected}");
        assert!(test_eq(result, expected));
    }
}

fn test_unspecified_precision_function(expression: &str, function: fn(f32) -> f32) {
    let func = compile_expression!(expression, (x) -> f32).unwrap();
    for (c, test_value) in TEST_VALUES {
        let result = func(test_value);
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
fn min() {
    test_function("min(x, 1)", |x| x.min(1.0));
}

#[test]
fn max() {
    test_function("max(x, 1)", |x| x.max(1.0));
}

#[test]
fn floor() {
    test_function("floor(x)", f32::floor);
}

#[test]
fn round() {
    test_function("round(x)", f32::round);
}

#[test]
fn ceil() {
    test_function("ceil(x)", f32::ceil);
}

#[test]
fn if_function() {
    let func = compile_expression!("if(is_normal(x), x, 0.0)", (x) -> f32).unwrap();
    let result = func(2.5);
    assert_eq!(result, 2.5);
    let result = func(f32::INFINITY);
    assert_eq!(result, 0.0);
}

#[test]
fn is_nan() {
    let func = compile_expression!("is_nan(x)", (x) -> f32).unwrap();
    let result = func(f32::NAN);
    assert!(result == 1.0);
    let result = func(1.0);
    assert!(result == 0.0);
}

#[test]
fn is_finite() {
    let func = compile_expression!("is_finite(x)", (x) -> f32).unwrap();
    let result = func(1.0);
    assert!(result == 1.0);
    let result = func(f32::INFINITY);
    assert!(result == 0.0);
    let result = func(f32::NAN);
    assert!(result == 0.0);
}

#[test]
fn is_infinite() {
    let func = compile_expression!("is_infinite(x)", (x) -> f32).unwrap();
    let result = func(f32::INFINITY);
    assert!(result == 1.0);
    let result = func(f32::NEG_INFINITY);
    assert!(result == 1.0);
    let result = func(f32::NAN);
    assert!(result == 0.0);
    let result = func(1.0);
    assert!(result == 0.0);
    let _ = 1_f32.is_infinite();
}

#[test]
fn is_normal() {
    let func = compile_expression!("is_normal(x)", (x) -> f32).unwrap();
    let result = func(MIN);
    assert!(result == 1.0);
    let result = func(MAX);
    assert!(result == 1.0);
    let result = func(ZERO);
    assert!(result == 0.0);
    let result = func(LOWER_THAN_MIN);
    assert!(result == 0.0);
    let result = func(f32::INFINITY);
    assert!(result == 0.0);
    let result = func(f32::NAN);
    assert!(result == 0.0);
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
fn pow() {
    test_unspecified_precision_function("pow(x, 2.5)", |x| x.powf(2.5));
}

#[test]
fn hypot() {
    test_unspecified_precision_function("hypot(x, 2.5)", |x| x.hypot(2.5));
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
    test_function("abs(x)", f32::abs);
}
