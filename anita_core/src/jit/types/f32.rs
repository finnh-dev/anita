use cranelift::prelude::{types::F32, FloatCC, FunctionBuilder, InstBuilder, Type, Value};

use super::AnitaType;

impl AnitaType for f32 {
    fn cranelift_repr() -> Type {
        F32
    }

    fn constant(builder: &mut FunctionBuilder, value: f32) -> Value {
        builder.ins().f32const(value)
    }

    fn add(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fadd(lhs, rhs)
    }

    fn sub(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fsub(lhs, rhs)
    }

    fn mul(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fmul(lhs, rhs)
    }

    fn div(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fdiv(lhs, rhs)
    }

    fn modulo(builder: &mut FunctionBuilder, value: Value, modulus: Value) -> Value {
        let div = builder.ins().fdiv(value, modulus);
        let trunc = builder.ins().trunc(div);
        let full_div = builder.ins().fmul(trunc, modulus);
        builder.ins().fsub(value, full_div)
    }

    fn neg(builder: &mut FunctionBuilder, value: Value) -> Value {
        builder.ins().fneg(value)
    }

    fn eq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::Equal, lhs, rhs)
    }

    fn neq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs)
    }

    fn gt(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs)
    }

    fn lt(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)
    }

    fn geq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs)
    }

    fn leq(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs)
    }

    fn and(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        let zero = builder.ins().f32const(0.0);
        let two = builder.ins().f32const(2.0);
        let lhs = builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
        let rhs = builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
        let intermediate = builder.ins().fadd(lhs, rhs);
        builder.ins().fcmp(FloatCC::Equal, intermediate, two)
    }

    fn or(builder: &mut FunctionBuilder, lhs: Value, rhs: Value) -> Value {
        let zero = builder.ins().f32const(0.0);
        let lhs = builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
        let rhs = builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
        let intermediate = builder.ins().fadd(lhs, rhs);
        builder.ins().fcmp(FloatCC::NotEqual, intermediate, zero)
    }

    fn not(builder: &mut FunctionBuilder, value: Value) -> Value {
        let zero = builder.ins().f32const(0.0);
        builder.ins().fcmp(FloatCC::Equal, value, zero)
    }

    extern "C" fn inbuilt_pow(self, value: Self) -> Self {
        self.powf(value)
    }
}
