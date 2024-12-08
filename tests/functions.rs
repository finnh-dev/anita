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

fn test_compiled_functiontion(expression: &str, compiled_functiontion: fn(f32) -> f32) {
    let compiled_function = compile_expression!(expression, (x) -> f32).unwrap();
    for (c, test_value) in TEST_VALUES {
        let result = compiled_function(test_value);
        let expected = compiled_functiontion(test_value);
        println!("{c}: {result} == {expected}");
        assert!(test_eq(result, expected));
    }
}

fn test_unspecified_precision_compiled_functiontion(expression: &str, compiled_functiontion: fn(f32) -> f32) {
    let compiled_function = compile_expression!(expression, (x) -> f32).unwrap();
    for (c, test_value) in TEST_VALUES {
        let result = compiled_function(test_value);
        let expected = compiled_functiontion(test_value);
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
    test_compiled_functiontion("min(x, 1)", |x| x.min(1.0));
}

#[test]
fn max() {
    test_compiled_functiontion("max(x, 1)", |x| x.max(1.0));
}

#[test]
fn floor() {
    test_compiled_functiontion("floor(x)", f32::floor);
}

#[test]
fn round() {
    test_compiled_functiontion("round(x)", f32::round);
}

#[test]
fn ceil() {
    test_compiled_functiontion("ceil(x)", f32::ceil);
}

#[test]
fn if_compiled_functiontion() {
    let compiled_function = compile_expression!("if(is_normal(x), x, 0.0)", (x) -> f32).unwrap();
    let result = compiled_function(2.5);
    assert_eq!(result, 2.5);
    let result = compiled_function(f32::INFINITY);
    assert_eq!(result, 0.0);
}

#[test]
fn is_nan() {
    let compiled_function = compile_expression!("is_nan(x)", (x) -> f32).unwrap();
    let result = compiled_function(f32::NAN);
    assert!(result == 1.0);
    let result = compiled_function(1.0);
    assert!(result == 0.0);
}

#[test]
fn is_finite() {
    let compiled_function = compile_expression!("is_finite(x)", (x) -> f32).unwrap();
    let result = compiled_function(1.0);
    assert!(result == 1.0);
    let result = compiled_function(f32::INFINITY);
    assert!(result == 0.0);
    let result = compiled_function(f32::NAN);
    assert!(result == 0.0);
}

#[test]
fn is_infinite() {
    let compiled_function = compile_expression!("is_infinite(x)", (x) -> f32).unwrap();
    let result = compiled_function(f32::INFINITY);
    assert!(result == 1.0);
    let result = compiled_function(f32::NEG_INFINITY);
    assert!(result == 1.0);
    let result = compiled_function(f32::NAN);
    assert!(result == 0.0);
    let result = compiled_function(1.0);
    assert!(result == 0.0);
    let _ = 1_f32.is_infinite();
}

#[test]
fn is_normal() {
    let compiled_function = compile_expression!("is_normal(x)", (x) -> f32).unwrap();
    let result = compiled_function(MIN);
    assert!(result == 1.0);
    let result = compiled_function(MAX);
    assert!(result == 1.0);
    let result = compiled_function(ZERO);
    assert!(result == 0.0);
    let result = compiled_function(LOWER_THAN_MIN);
    assert!(result == 0.0);
    let result = compiled_function(f32::INFINITY);
    assert!(result == 0.0);
    let result = compiled_function(f32::NAN);
    assert!(result == 0.0);
}

#[test]
fn ln() {
    test_unspecified_precision_compiled_functiontion("ln(x)", f32::ln);
}

#[test]
fn log2() {
    test_unspecified_precision_compiled_functiontion("log2(x)", f32::log2);
}

#[test]
fn log10() {
    test_unspecified_precision_compiled_functiontion("log10(x)", f32::log10);
}

#[test]
fn exp() {
    test_unspecified_precision_compiled_functiontion("exp(x)", f32::exp);
}

#[test]
fn exp2() {
    test_unspecified_precision_compiled_functiontion("exp2(x)", f32::exp2);
}

#[test]
fn pow() {
    test_unspecified_precision_compiled_functiontion("pow(x, 2.5)", |x| x.powf(2.5));
}

#[test]
fn hypot() {
    test_unspecified_precision_compiled_functiontion("hypot(x, 2.5)", |x| x.hypot(2.5));
}

#[test]
fn cos() {
    test_unspecified_precision_compiled_functiontion("cos(x)", f32::cos);
}

#[test]
fn acos() {
    test_unspecified_precision_compiled_functiontion("acos(x)", f32::acos);
}

#[test]
fn cosh() {
    test_unspecified_precision_compiled_functiontion("cosh(x)", f32::cosh);
}

#[test]
fn acosh() {
    test_unspecified_precision_compiled_functiontion("acosh(x)", f32::acosh);
}

#[test]
fn sin() {
    test_unspecified_precision_compiled_functiontion("sin(x)", f32::sin);
}

#[test]
fn asin() {
    test_unspecified_precision_compiled_functiontion("asin(x)", f32::asin);
}

#[test]
fn sinh() {
    test_unspecified_precision_compiled_functiontion("sinh(x)", f32::sinh);
}

#[test]
fn asinh() {
    test_unspecified_precision_compiled_functiontion("asinh(x)", f32::asinh);
}

#[test]
fn tan() {
    test_unspecified_precision_compiled_functiontion("tan(x)", f32::tan);
}

#[test]
fn atan() {
    test_unspecified_precision_compiled_functiontion("atan(x)", f32::atan);
}

#[test]
fn tanh() {
    test_unspecified_precision_compiled_functiontion("tanh(x)", f32::tanh);
}

#[test]
fn atanh() {
    test_unspecified_precision_compiled_functiontion("atanh(x)", f32::atanh);
}

#[test]
fn sqrt() {
    test_unspecified_precision_compiled_functiontion("sqrt(x)", f32::sqrt);
}

#[test]
fn cbrt() {
    test_unspecified_precision_compiled_functiontion("cbrt(x)", f32::cbrt);
}

#[test]
fn abs() {
    test_compiled_functiontion("abs(x)", f32::abs);
}
