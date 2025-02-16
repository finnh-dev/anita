use cranelift::prelude::{types::F32X4, FloatCC, InstBuilder};

use super::AnitaType;
use std::simd::{f32x4, StdFloat};

impl AnitaType for f32x4 {
    fn cranelift_repr() -> cranelift::prelude::Type {
        F32X4
    }

    fn constant(
        builder: &mut cranelift::prelude::FunctionBuilder,
        value: f32,
    ) -> cranelift::prelude::Value {
        let scalar_value = builder.ins().f32const(value);
        builder.ins().splat(F32X4, scalar_value)
    }

    fn add(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fadd(lhs, rhs)
    }

    fn sub(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fsub(lhs, rhs)
    }

    fn mul(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fmul(lhs, rhs)
    }

    fn div(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fdiv(lhs, rhs)
    }

    fn modulo(
        builder: &mut cranelift::prelude::FunctionBuilder,
        value: cranelift::prelude::Value,
        modulus: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        let div = builder.ins().fdiv(value, modulus);
        let trunc = builder.ins().trunc(div);
        let full_div = builder.ins().fmul(trunc, modulus);
        builder.ins().fsub(value, full_div)
    }

    fn neg(
        builder: &mut cranelift::prelude::FunctionBuilder,
        value: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fneg(value)
    }

    fn eq(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::Equal, lhs, rhs)
    }

    fn neq(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs)
    }

    fn gt(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs)
    }

    fn lt(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)
    }

    fn geq(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs)
    }

    fn leq(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs)
    }

    fn and(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        let zero_scalar = builder.ins().f32const(0.0);
        let zero = builder.ins().splat(F32X4, zero_scalar);
        let two_scalar = builder.ins().f32const(2.0);
        let two = builder.ins().splat(F32X4, two_scalar);
        let lhs = builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
        let rhs = builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
        let intermediate = builder.ins().fadd(lhs, rhs);
        builder.ins().fcmp(FloatCC::Equal, intermediate, two)
    }

    fn or(
        builder: &mut cranelift::prelude::FunctionBuilder,
        lhs: cranelift::prelude::Value,
        rhs: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        let zero_scalar = builder.ins().f32const(0.0);
        let zero = builder.ins().splat(F32X4, zero_scalar);
        let lhs = builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
        let rhs = builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
        let intermediate = builder.ins().fadd(lhs, rhs);
        builder.ins().fcmp(FloatCC::NotEqual, intermediate, zero)
    }

    fn not(
        builder: &mut cranelift::prelude::FunctionBuilder,
        value: cranelift::prelude::Value,
    ) -> cranelift::prelude::Value {
        let zero_scalar = builder.ins().f32const(0.0);
        let zero = builder.ins().splat(F32X4, zero_scalar);
        builder.ins().fcmp(FloatCC::Equal, value, zero)
    }
    
    #[allow(improper_ctypes_definitions)] // TODO: verify safety
    extern "C" fn inbuilt_pow(self, value: Self) -> Self {
        f32x4::exp(value * self.ln())
    }
}
