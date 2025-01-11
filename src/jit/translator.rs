use std::collections::HashMap;

use cranelift::{
    codegen::ir::FuncRef,
    prelude::{FloatCC, FunctionBuilder, InstBuilder, Value, Variable},
};
use cranelift_jit::JITModule;
use cranelift_module::{Module, ModuleError};

use super::{super::function_manager::FunctionManager, frontend::Expr};

pub(super) struct ExprTranslator<'a, 'b, F: FunctionManager> {
    pub(super) builder: &'b mut FunctionBuilder<'a>,
    pub(super) variables: HashMap<String, Variable>,
    pub(super) functions: HashMap<String, (FuncRef, usize)>,
    pub(super) module: &'b mut JITModule,
    pub(super) _function_manager: std::marker::PhantomData<F>,
}

// TODO: improve Errors
#[derive(Debug)]
pub enum TranslatorError {
    ExpressionEvaluatesToNoValue(Expr),
    FunctionNotFound(String),
    ModuleError(ModuleError),
}

impl From<ModuleError> for TranslatorError {
    fn from(value: ModuleError) -> Self {
        Self::ModuleError(value)
    }
}

impl<'a, 'b, F: FunctionManager> ExprTranslator<'a, 'b, F> {
    // pub fn get_builder(
    //     self,
    // ) ->
    //     FunctionBuilder<'a>
    // {
    //     self.builder
    // }

    pub fn translate(&mut self, expr: Expr) -> Result<Option<Value>, TranslatorError> {
        match expr {
            Expr::VariableRead { identifier } => {
                let variable = self
                    .variables
                    .get(&identifier)
                    .unwrap_or_else(|| panic!("Variable {} does not exist", identifier));
                Ok(Some(self.builder.use_var(*variable)))
            }
            Expr::Const { value } => Ok(Some(self.builder.ins().f32const(value))),
            Expr::Chain { side, ret } => {
                let _side = self.translate(*side)?;
                let ret = self.get_value(*ret)?;
                Ok(Some(ret))
            }
            Expr::Call { identifier, args } => {
                let args = args
                    .into_iter()
                    .try_fold(Vec::new(), |mut acc, expr| {
                        match self.translate(expr)? {
                            Some(val) => {
                                acc.push(val);
                                Result::<Vec<Value>, TranslatorError>::Ok(acc)
                            }
                            None => Ok(acc),
                        }
                    })?;

                Ok(Some(
                    self.function_call(&identifier, args.as_slice())?,
                ))
            }
            Expr::Add { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fadd(lhs, rhs)))
            }
            Expr::Sub { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fsub(lhs, rhs)))
            }
            Expr::Mul { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fmul(lhs, rhs)))
            }
            Expr::Div { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fdiv(lhs, rhs)))
            }
            Expr::Mod { lhs, rhs } => {
                let (value, modulus) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                let div = self.builder.ins().fdiv(value, modulus);
                let trunc = self.builder.ins().trunc(div);
                let full_div = self.builder.ins().fmul(trunc, modulus);
                Ok(Some(self.builder.ins().fsub(value, full_div)))
            }
            Expr::Exp { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(
                    self.function_call("inbuilt_powf", &[lhs, rhs])?,
                ))
            }
            Expr::Neg { value } => {
                let value = self.get_value(*value)?;
                Ok(Some(self.builder.ins().fneg(value)))
            }
            Expr::Assign { identifier, value } => {
                let variable = self
                    .variables
                    .get(&identifier)
                    .copied()
                    .unwrap_or_else(|| panic!("Variable {} does not exist", identifier));
                let value = self.get_value(*value)?;
                self.builder.def_var(variable, value);
                Ok(None)
            }
            Expr::Eq { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(FloatCC::Equal, lhs, rhs)))
            }
            Expr::Neq { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs)))
            }
            Expr::Gt { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::GreaterThan,
                    lhs,
                    rhs,
                )))
            }
            Expr::Lt { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)))
            }
            Expr::Geq { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::GreaterThanOrEqual,
                    lhs,
                    rhs,
                )))
            }
            Expr::Leq { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::LessThanOrEqual,
                    lhs,
                    rhs,
                )))
            }
            Expr::And { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                let zero = self.builder.ins().f32const(0.0);
                let two = self.builder.ins().f32const(2.0);
                let lhs = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
                let rhs = self.builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
                let intermediate = self.builder.ins().fadd(lhs, rhs);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::Equal,
                    intermediate,
                    two,
                )))
            }
            Expr::Or { lhs, rhs } => {
                let (lhs, rhs) = (self.get_value(*lhs)?, self.get_value(*rhs)?);
                let zero = self.builder.ins().f32const(0.0);
                let lhs = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
                let rhs = self.builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
                let intermediate = self.builder.ins().fadd(lhs, rhs);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::NotEqual,
                    intermediate,
                    zero,
                )))
            }
            Expr::Not { value } => {
                let value = self.get_value(*value)?;
                let zero = self.builder.ins().f32const(0.0);
                Ok(Some(self.builder.ins().fcmp(FloatCC::Equal, value, zero)))
            }
        }
    }

    fn get_value(&mut self, expr: Expr) -> Result<Value, TranslatorError> {
        let expr_copy = expr.clone();
        let Some(value) = self.translate(expr)? else {
            return Err(TranslatorError::ExpressionEvaluatesToNoValue(expr_copy));
        };
        Ok(value)
    }

    fn function_call(
        &mut self,
        identifier: &str,
        params: &[Value],
    ) -> Result<Value, TranslatorError> {
        let (func_ref, _) = self.declare_function(identifier)?;
        let call = self.builder.ins().call(func_ref, params);
        Ok(self.builder.inst_results(call)[0])
    }

    fn declare_function(
        &mut self,
        identifier: &str,
    ) -> Result<(FuncRef, usize), TranslatorError> {
        let Some(func) = self.functions.get(identifier) else {
            let Some(signature) =
                F::function_signature(identifier, self.module.isa().default_call_conv())
            else {
                return Err(TranslatorError::FunctionNotFound(identifier.to_owned()));
            };
            let func_id = self.module.declare_function(
                identifier,
                cranelift_module::Linkage::Import,
                &signature,
            )?;
            let func = (
                self.module.declare_func_in_func(func_id, self.builder.func),
                signature.params.len(),
            );
            self.functions.insert(identifier.into(), func);
            return Ok(func);
        };
        Ok(func.to_owned())
    }
}