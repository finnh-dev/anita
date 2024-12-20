use std::collections::HashMap;

use cranelift::{
    codegen::ir::FuncRef,
    prelude::{types::I64, FloatCC, FunctionBuilder, InstBuilder, Type, Value, Variable},
};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use evalexpr::{EvalexprError, Node};

use super::{super::function_manager::FunctionManager, frontend::Expr, EvalexprCompError};

pub(super) struct ExprTranslator<'a, F: FunctionManager> {
    pub(super) builder: FunctionBuilder<'a>,
    pub(super) variables: HashMap<String, Variable>,
    pub(super) functions: HashMap<String, (FuncRef, usize)>,
    pub(super) module: &'a mut JITModule,
    pub(super) _function_manager: std::marker::PhantomData<F>,
}

// TODO: improve Errors
#[derive(Debug)]
pub enum TranslatorError {
    ExpressionEvaluatesToNoValue(Expr),
}

impl<'a, F: FunctionManager> ExprTranslator<'a, F> {
    pub fn deconstruct(
        self,
    ) -> (
        FunctionBuilder<'a>,
        HashMap<String, Variable>,
        HashMap<String, (FuncRef, usize)>,
        &'a mut JITModule,
    ) {
        (self.builder, self.variables, self.functions, self.module)
    }

    pub fn translate_frontend(&mut self, expr: Expr) -> Result<Option<Value>, TranslatorError> {
        let expr_copy = expr.clone(); // TODO: Fix
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
                let _side = self.translate_frontend(*side)?;
                let ret = self.get_value(*ret)?;
                Ok(Some(ret))
            }
            Expr::Call { identifier, args } => {
                let Some(args) = args
                    .into_iter()
                    .try_fold(Some(Vec::new()), |mut acc, expr| {
                        match self.translate_frontend(expr)? {
                            Some(val) => {
                                acc.as_mut().unwrap().push(val);
                                Ok(acc)
                            }
                            None => Ok(acc),
                        }
                    })?
                else {
                    return Err(TranslatorError::ExpressionEvaluatesToNoValue(expr_copy));
                };

                Ok(Some(
                    self.function_call(&identifier, args.as_slice()).unwrap(),
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
                    self.function_call("inbuilt_powf", &[lhs, rhs]).unwrap(),
                )) // TODO: FIX!
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
                let one = self.builder.ins().f32const(1.0);
                let lhs = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
                let rhs = self.builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
                let intermediate = self.builder.ins().fadd(lhs, rhs);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::Equal,
                    intermediate,
                    one,
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
        let Some(value) = self.translate_frontend(expr)? else {
            return Err(TranslatorError::ExpressionEvaluatesToNoValue(expr_copy));
        };
        Ok(value)
    }

    pub fn translate_operator(&mut self, node: &Node) -> Result<Option<Value>, EvalexprCompError> {
        match node.operator() {
            evalexpr::Operator::RootNode => {
                let children = node.children();
                if children.len() > 1 {
                    return Err(EvalexprCompError::MalformedOperatorTree(node.clone()));
                }

                if let Some(op_tree) = children.first() {
                    self.translate_operator(op_tree)
                } else {
                    Ok(None)
                }
            }
            evalexpr::Operator::Add => {
                let (lhs, rhs) = self.binary_operation(node)?;
                let operation_type = self.check_value_type(lhs);
                let rhs = self.convert_value_type(operation_type, rhs)?;
                match operation_type {
                    op_type if op_type.is_int() => Ok(Some(self.builder.ins().iadd(lhs, rhs))),
                    op_type if op_type.is_float() => Ok(Some(self.builder.ins().fadd(lhs, rhs))),
                    op_type => Err(EvalexprCompError::UseOfUnsupportedType(op_type)),
                }
            }
            evalexpr::Operator::Sub => {
                let (lhs, rhs) = self.binary_operation(node)?;
                let operation_type = self.check_value_type(lhs);
                let rhs = self.convert_value_type(operation_type, rhs)?;
                match operation_type {
                    op_type if op_type.is_int() => Ok(Some(self.builder.ins().isub(lhs, rhs))),
                    op_type if op_type.is_float() => Ok(Some(self.builder.ins().fsub(lhs, rhs))),
                    op_type => Err(EvalexprCompError::UseOfUnsupportedType(op_type)),
                }
            }
            evalexpr::Operator::Neg => {
                let operand = self.unary_operation(node)?;
                match self.check_value_type(operand) {
                    op_type if op_type.is_int() => Ok(Some(self.builder.ins().ineg(operand))),
                    op_type if op_type.is_float() => Ok(Some(self.builder.ins().fneg(operand))),
                    op_type => Err(EvalexprCompError::UseOfUnsupportedType(op_type)),
                }
            }
            evalexpr::Operator::Mul => {
                let (lhs, rhs) = self.binary_operation(node)?;
                let operation_type = self.check_value_type(lhs);
                let rhs = self.convert_value_type(operation_type, rhs)?;
                match operation_type {
                    op_type if op_type.is_int() => Ok(Some(self.builder.ins().imul(lhs, rhs))),
                    op_type if op_type.is_float() => Ok(Some(self.builder.ins().fmul(lhs, rhs))),
                    op_type => Err(EvalexprCompError::UseOfUnsupportedType(op_type)),
                }
            }
            evalexpr::Operator::Div => {
                let (lhs, rhs) = self.binary_operation(node)?;
                let operation_type = self.check_value_type(lhs);
                let rhs = self.convert_value_type(operation_type, rhs)?;
                match operation_type {
                    op_type if op_type.is_int() => Ok(Some(self.builder.ins().sdiv(lhs, rhs))),
                    op_type if op_type.is_float() => Ok(Some(self.builder.ins().fdiv(lhs, rhs))),
                    op_type => Err(EvalexprCompError::UseOfUnsupportedType(op_type)),
                }
            }
            evalexpr::Operator::Mod => {
                let (value, modulus) = self.binary_operation(node)?;
                let div = self.builder.ins().fdiv(value, modulus);
                let trunc = self.builder.ins().trunc(div);
                let full_div = self.builder.ins().fmul(trunc, modulus);
                Ok(Some(self.builder.ins().fsub(value, full_div)))
            }
            evalexpr::Operator::Exp => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.function_call("inbuilt_powf", &[lhs, rhs])?))
            }
            evalexpr::Operator::Eq => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(FloatCC::Equal, lhs, rhs)))
            }
            evalexpr::Operator::Neq => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs)))
            }
            evalexpr::Operator::Gt => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::GreaterThan,
                    lhs,
                    rhs,
                )))
            }
            evalexpr::Operator::Lt => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)))
            }
            evalexpr::Operator::Geq => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::GreaterThanOrEqual,
                    lhs,
                    rhs,
                )))
            }
            evalexpr::Operator::Leq => {
                let (lhs, rhs) = self.binary_operation(node)?;
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::LessThanOrEqual,
                    lhs,
                    rhs,
                )))
            }
            evalexpr::Operator::And => {
                let (lhs, rhs) = self.binary_operation(node)?;
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
            evalexpr::Operator::Or => {
                let (lhs, rhs) = self.binary_operation(node)?;
                let zero = self.builder.ins().f32const(0.0);
                let one = self.builder.ins().f32const(1.0);
                let lhs = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, zero);
                let rhs = self.builder.ins().fcmp(FloatCC::NotEqual, rhs, zero);
                let intermediate = self.builder.ins().fadd(lhs, rhs);
                Ok(Some(self.builder.ins().fcmp(
                    FloatCC::Equal,
                    intermediate,
                    one,
                )))
            }
            evalexpr::Operator::Not => {
                let value = self.unary_operation(node)?;
                let zero = self.builder.ins().f32const(0.0);
                Ok(Some(self.builder.ins().fcmp(FloatCC::Equal, value, zero)))
            }
            evalexpr::Operator::Assign => {
                let children = node.children();
                if children.len() != 2 {
                    return Err(EvalexprError::WrongOperatorArgumentAmount {
                        expected: 2,
                        actual: children.len(),
                    }
                    .into());
                }

                let (target, value_ast) = (&children[0], &children[1]);
                let Some(value) = self.translate_operator(value_ast)? else {
                    return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                        value_ast.clone(),
                    ));
                };

                let evalexpr::Operator::VariableIdentifierWrite { identifier } = target.operator()
                else {
                    return Err(EvalexprCompError::MalformedOperatorTree(node.clone()));
                };
                let variable = self
                    .variables
                    .get(identifier)
                    .copied()
                    .ok_or_else(|| EvalexprCompError::VariableNotFound(identifier.clone()))?;

                let var_value = self.builder.use_var(variable);
                let var_type = self.check_value_type(var_value);

                let value = self.convert_value_type(var_type, value)?;

                self.builder.def_var(variable, value);

                Ok(None)
            }
            evalexpr::Operator::AddAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::SubAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::MulAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::DivAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::ModAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::ExpAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::AndAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::OrAssign => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::Tuple => Err(EvalexprCompError::UnsupportedOperator(
                node.operator().clone(),
            )),
            evalexpr::Operator::Chain => {
                let mut return_value = None;
                for ast in node.children() {
                    return_value = self.translate_operator(ast)?;
                }
                Ok(return_value)
            }
            evalexpr::Operator::Const { value } => match value {
                evalexpr::Value::String(_) => Err(EvalexprCompError::EvalexprError(
                    EvalexprError::CustomMessage("String is not supported".to_owned()),
                )),
                evalexpr::Value::Float(value) => {
                    Ok(Some(self.builder.ins().f32const(*value as f32)))
                }
                evalexpr::Value::Int(value) => Ok(Some(self.builder.ins().f32const(*value as f32))),
                evalexpr::Value::Boolean(value) => {
                    Ok(Some(self.builder.ins().iconst(I64, *value as i64)))
                }
                evalexpr::Value::Tuple(_) => Err(EvalexprCompError::EvalexprError(
                    EvalexprError::CustomMessage("Tuple is not supported".to_owned()),
                )),
                evalexpr::Value::Empty => Err(EvalexprCompError::EvalexprError(
                    EvalexprError::CustomMessage("Empty is not supported".to_owned()),
                )),
            },
            evalexpr::Operator::VariableIdentifierWrite { identifier: _ } => {
                unreachable!("VariableWrite should be handled in assignment")
            }
            evalexpr::Operator::VariableIdentifierRead { identifier } => {
                let variable = self
                    .variables
                    .get(identifier)
                    .unwrap_or_else(|| panic!("Variable {} does not exist", identifier));
                Ok(Some(self.builder.use_var(*variable)))
            }
            evalexpr::Operator::FunctionIdentifier { identifier } => {
                let Some(root) = node.children().first() else {
                    return Err(EvalexprCompError::MalformedOperatorTree(node.clone()));
                };
                if *root.operator() != evalexpr::Operator::RootNode {
                    return Err(EvalexprCompError::MalformedOperatorTree(node.clone()));
                };
                let Some(maybe_tuple) = root.children().first() else {
                    return Err(EvalexprCompError::MalformedOperatorTree(node.clone()));
                };
                let arguments = match maybe_tuple.operator() {
                    evalexpr::Operator::Tuple => {
                        let mut arguments = Vec::new();
                        for node in maybe_tuple.children() {
                            arguments.push(self.translate_operator(node)?);
                        }
                        arguments
                            .iter()
                            .map(|val| {
                                val.ok_or_else(|| {
                                    EvalexprCompError::ExpressionEvaluatesToNoValue(
                                        maybe_tuple.clone(),
                                    )
                                })
                            })
                            .collect::<Result<Box<[Value]>, EvalexprCompError>>()?
                    }
                    _ => {
                        let Some(value) = self.translate_operator(maybe_tuple)? else {
                            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                                maybe_tuple.clone(),
                            ));
                        };
                        Box::new([value])
                    }
                };

                Ok(Some(self.function_call(&identifier, &arguments)?))
            }
        }
    }

    fn function_call(
        &mut self,
        identifier: &str,
        params: &[Value],
    ) -> Result<Value, EvalexprCompError> {
        let (func_ref, _) = self.declare_function(identifier)?;
        let call = self.builder.ins().call(func_ref, &params);
        Ok(self.builder.inst_results(call)[0])
    }

    fn declare_function(
        &mut self,
        identifier: &str,
    ) -> Result<(FuncRef, usize), EvalexprCompError> {
        let Some(func) = self.functions.get(identifier) else {
            let Some(signature) =
                F::function_signature(identifier, self.module.isa().default_call_conv())
            else {
                return Err(EvalexprCompError::FunctionNotFound(identifier.to_owned()));
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

    fn binary_operation(&mut self, node: &Node) -> Result<(Value, Value), EvalexprCompError> {
        let children = node.children();
        if children.len() != 2 {
            return Err(EvalexprError::WrongOperatorArgumentAmount {
                expected: 2,
                actual: children.len(),
            }
            .into());
        }

        let (lhs_ast, rhs_ast) = (&children[0], &children[1]);
        let Some(lhs) = self.translate_operator(lhs_ast)? else {
            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                lhs_ast.clone(),
            ));
        };
        let Some(rhs) = self.translate_operator(rhs_ast)? else {
            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                rhs_ast.clone(),
            ));
        };

        Ok((lhs, rhs))
    }

    fn unary_operation(&mut self, node: &Node) -> Result<Value, EvalexprCompError> {
        let children = node.children();
        if children.len() != 1 {
            return Err(EvalexprError::WrongOperatorArgumentAmount {
                expected: 2,
                actual: children.len(),
            }
            .into());
        }

        let operand_ast = &children[0];
        let Some(operand) = self.translate_operator(operand_ast)? else {
            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                operand_ast.clone(),
            ));
        };

        Ok(operand)
    }

    fn check_value_type(&self, value: Value) -> Type {
        let dfg = &self.builder.func.dfg;
        dfg.value_type(value)
    }

    pub(super) fn convert_value_type(
        &mut self,
        target_type: Type,
        value: Value,
    ) -> Result<Value, EvalexprCompError> {
        match (target_type, self.check_value_type(value)) {
            (target_type, source_type) if target_type == source_type => Ok(value),
            // (target_type, source_type)
            //     if target_type.is_int()
            //         && source_type.is_int()
            //         && target_type.bits() > source_type.bits() =>
            // {
            //     Ok(self.builder.ins().sextend(target_type, value))
            // }
            // (target_type, source_type)
            //     if target_type.is_int()
            //         && source_type.is_int()
            //         && target_type.bits() < source_type.bits() =>
            // {
            //     Ok(self.builder.ins().ireduce(target_type, value))
            // }
            // (target_type, source_type) if target_type.is_int() && source_type.is_float() => {
            //     Ok(self.builder.ins().fcvt_to_sint(target_type, value))
            // }
            // (target_type, source_type) if target_type.is_float() && source_type.is_int() => {
            //     Ok(self.builder.ins().fcvt_from_sint(target_type, value))
            // }
            // (target_type, source_type)
            //     if target_type.is_float()
            //         && source_type.is_float()
            //         && target_type.bits() > source_type.bits() =>
            // {
            //     Ok(self.builder.ins().fpromote(target_type, value))
            // }
            // (target_type, source_type)
            //     if target_type.is_float()
            //         && source_type.is_float()
            //         && target_type.bits() > source_type.bits() =>
            // {
            //     Ok(self.builder.ins().fdemote(target_type, value))
            // }
            (target_type, source_type) => Err(EvalexprCompError::UnsupportedTypeConversion {
                target_type,
                source_type,
            }),
        }
    }
}
