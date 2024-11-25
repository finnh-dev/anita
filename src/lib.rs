#![deny(unused_must_use)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod jit;

#[cfg(test)]
mod tests {
    use core::f32;

    use evalexpr::EvalexprError;
    use jit::EvalexprCompError;

    use super::*;

    #[test]
    fn owned_input() {
        let input = "x".to_owned();
        let f = compile_expression!(input, (x) -> f32).unwrap();
        let result = f.execute(1.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn empty_input() {
        let result = compile_expression!("", (x) -> f32);
        if let Err(EvalexprCompError::EvalexprError(e)) = result {
            assert_eq!(
                e,
                EvalexprError::WrongOperatorArgumentAmount {
                    expected: 2,
                    actual: 0
                }
            );
        } else {
            panic!("Expected EvalexprError but got: {:?}", result)
        }
    }

    #[test]
    fn exponentiation() {
        let func = compile_expression!("x ^ 2", (x) -> f32).unwrap();
        let result = func.execute(2.0);
        assert_eq!(result, 2.0_f32.powf(2.0));
    }

    #[test]
    fn product() {
        let func = compile_expression!("x * 2", (x) -> f32).unwrap();
        let result = func.execute(1.5);
        assert_eq!(result, 1.5 * 2.0);
    }

    #[test]
    fn fdivision() {
        let func = compile_expression!("x / 2", (x) -> f32).unwrap();
        let result = func.execute(2.0);
        assert_eq!(result, 2.0 / 2.0);
    }

    #[test]
    #[ignore = "integer division by zero crashes"]
    fn idivision_by_zero() {
        let func = compile_expression!("0.0 + 1 / x", (x) -> f32).unwrap();
        let result = func.execute(0.0);
        assert!(result.is_infinite());
    }

    #[test]
    fn fdivision_by_zero() {
        let func = compile_expression!("1.0 / x", (x) -> f32).unwrap();
        let result = func.execute(0.0);
        assert!(result.is_infinite());
    }

    #[test]
    fn modulo() {
        let func = compile_expression!("x % 2", (x) -> f32).unwrap();
        let result = func.execute(3.0);
        assert_eq!(result, 3.0 % 2.0);
    }

    #[test]
    fn sum() {
        let func = compile_expression!("x + 2", (x) -> f32).unwrap();
        let result = func.execute(5.0);
        assert_eq!(result, 5.0 + 2.0);
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

    #[test]
    fn difference() {
        let func = compile_expression!("x - 2", (x) -> f32).unwrap();
        let result = func.execute(5.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn chain() {
        let func = compile_expression!("y = 2; z = 2; x + y + z", (x) -> f32).unwrap();
        let result = func.execute(2.0);
        assert_eq!(result, 2.0 + 2.0 + 2.0);
    }

    // TODO: Test against EvalExpr
    // TODO: Automate testing with value collections and randomized values
}
