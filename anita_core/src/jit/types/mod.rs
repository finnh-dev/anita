use cranelift::prelude::{FunctionBuilder, Type, Value};

pub trait AnitaType {
    fn cranelift_repr() -> Type;

    fn constant(builder: &mut FunctionBuilder, value: f32) -> Value;
    fn add(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn sub(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn mul(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn div(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn modulo(builder: &mut FunctionBuilder, value: Value, modulus: Value) -> Value;
    fn neg(builder: &mut FunctionBuilder, value: Value) -> Value;
    fn eq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn neq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn gt(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn lt(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn geq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn leq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn and(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn or(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value;
    fn not(builder: &mut FunctionBuilder, value: Value) -> Value;

    extern "C" fn inbuilt_pow(self, value: Self) -> Self;
}

mod f32;
mod f64;