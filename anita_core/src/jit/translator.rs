use std::collections::HashMap;

use cranelift::{
    codegen::ir::FuncRef,
    prelude::{FunctionBuilder, InstBuilder, Value, Variable},
};
use cranelift_jit::JITModule;
use cranelift_module::{Module, ModuleError};

use super::{super::function_manager::FunctionManager, frontend::Expr, types::AnitaType};

pub(super) struct ExprTranslator<'a, 'b, T: AnitaType, F: FunctionManager> {
    pub(super) builder: &'b mut FunctionBuilder<'a>,
    pub(super) variables: HashMap<String, Variable>,
    pub(super) functions: HashMap<String, (FuncRef, usize)>,
    pub(super) module: &'b mut JITModule,
    pub(super) _function_manager: std::marker::PhantomData<F>,
    pub(super) _type: std::marker::PhantomData<T>,
}

// TODO: improve Errors
#[derive(Debug)]
pub enum TranslatorError {
    FunctionNotFound(String),
    ModuleError(ModuleError),
}

impl From<ModuleError> for TranslatorError {
    fn from(value: ModuleError) -> Self {
        Self::ModuleError(value)
    }
}

impl<T: AnitaType, F: FunctionManager> ExprTranslator<'_, '_, T, F> {
    pub fn translate(&mut self, expr: Expr) -> Result<Value, TranslatorError> {
        match expr {
            Expr::VariableRead { identifier } => {
                let variable = self
                    .variables
                    .get(&identifier)
                    .unwrap_or_else(|| panic!("Variable {} does not exist", identifier));
                Ok(self.builder.use_var(*variable))
            }
            Expr::Const { value } => Ok(T::constant(self.builder, value)),
            Expr::Chain { side, ret } => {
                let _side = self.translate(*side)?;
                let ret = self.translate(*ret)?;
                Ok(ret)
            }
            Expr::Call { identifier, args } => {
                let args: Vec<Value> = args.into_iter().map(|expr| {
                    self.translate(expr)
                }).collect::<Result<Vec<Value>, TranslatorError>>()?;

                Ok(self.function_call(&identifier, args.as_slice())?)
            }
            Expr::Add { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::add(self.builder, lhs, rhs))
            }
            Expr::Sub { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::sub(self.builder, lhs, rhs))
            }
            Expr::Mul { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::mul(self.builder, lhs, rhs))
            }
            Expr::Div { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::div(self.builder, lhs, rhs))
            }
            Expr::Mod { lhs, rhs } => {
                let (value, modulus) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::modulo(self.builder, value, modulus))
            }
            Expr::Exp { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(self.function_call("inbuilt_pow", &[lhs, rhs])?)
            }
            Expr::Neg { value } => {
                let value = self.translate(*value)?;
                Ok(T::neg(self.builder, value))
            }
            Expr::Assign { identifier, value } => {
                let variable = self
                    .variables
                    .get(&identifier)
                    .copied()
                    .unwrap_or_else(|| panic!("Variable {} does not exist", identifier));
                let value = self.translate(*value)?;
                self.builder.def_var(variable, value);
                Ok(self.builder.use_var(variable))
            }
            Expr::Eq { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::eq(self.builder, lhs, rhs))
            }
            Expr::Neq { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::neq(self.builder, lhs, rhs))
            }
            Expr::Gt { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::gt(self.builder, lhs, rhs))
            }
            Expr::Lt { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::lt(self.builder, lhs, rhs))
            }
            Expr::Geq { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::geq(self.builder, lhs, rhs))
            }
            Expr::Leq { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::leq(self.builder, lhs, rhs))
            }
            Expr::And { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::and(self.builder, lhs, rhs))
            }
            Expr::Or { lhs, rhs } => {
                let (lhs, rhs) = (self.translate(*lhs)?, self.translate(*rhs)?);
                Ok(T::or(self.builder, lhs, rhs))
            }
            Expr::Not { value } => {
                let value = self.translate(*value)?;
                Ok(T::not(self.builder, value))
            }
        }
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

    fn declare_function(&mut self, identifier: &str) -> Result<(FuncRef, usize), TranslatorError> {
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
