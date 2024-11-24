#![deny(unused_must_use)]
#![deny(unsafe_op_in_unsafe_fn)]

use jit::{EvalexprCompError, EvalexprFunction, JIT};

mod jit;

pub fn compile_expr_one_param(expr: String) -> Result<EvalexprFunction<f32, f32>, EvalexprCompError> {
    let jit = JIT::default();

    let func = jit.compile(expr, &["x"])?;
    Ok(func)
}

pub fn compile_macro(expr: String) -> Result<EvalexprFunction<f32, f32>, EvalexprCompError> {
    let function = compile_expression!(expr, (x) -> (f32));
    let foo: compile_expression!(@to_f32 x) = 1.0;
    function
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    #[test]
    #[ignore]
    fn exponentiation() {
        todo!()
    }

    #[test]
    fn product() {
        let func = compile_expr_one_param("x * 2".into()).unwrap();
        let result = func.execute(1.5);
        assert_eq!(result, 3.0);
    }

    #[test]
    #[ignore]
    fn division() {
        todo!()
    }

    #[test]
    #[ignore]
    fn division_by_zero() {
        let func = compile_expr_one_param("1 / x".into()).unwrap();
        let result = func.execute(0.0);
        assert_eq!(result, f32::NAN);
    }

    #[test]
    #[ignore]
    fn modulo() {
        todo!()
    }

    #[test]
    fn sum() {
        let func = compile_expr_one_param("x + 2".into()).unwrap();
        let result = func.execute(5.0);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn sum_overflow() {
        let func = compile_expr_one_param("x + 1".into()).unwrap();
        let result = func.execute(f32::MAX);
        println!("result: {result}, expected: {}", f32::MAX + 1.0);
        assert_eq!(result, f32::MAX);
    }

    #[test]
    fn sum_nan() {
        let func = compile_expr_one_param("x + 1".into()).unwrap();
        let result = func.execute(f32::NAN);
        println!("result: {result}, expected: {}", f32::NAN);
        assert!(result.is_nan())
    }

    #[test]
    fn difference() {
        let func = compile_expr_one_param("x - 2".into()).unwrap();
        let result = func.execute(5.0);
        assert_eq!(result, 3.0);
    }
}
