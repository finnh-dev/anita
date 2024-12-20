use std::collections::HashMap;

use cranelift::{
    codegen::ir::FuncRef,
    prelude::{types::I64, FunctionBuilder, InstBuilder, Type, Value, Variable},
};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use evalexpr::{EvalexprError, Node};

use super::{super::function_manager::FunctionManager, EvalexprCompError};

pub(super) struct ExprTranslator<'a, F:FunctionManager> {
    pub(super) builder: FunctionBuilder<'a>,
    pub(super) variables: HashMap<String, Variable>,
    pub(super) functions: HashMap<String, (FuncRef, usize)>,
    pub(super) module: &'a mut JITModule,
    pub(super) _function_manager: std::marker::PhantomData<F>,
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
            },
            evalexpr::Operator::Exp => Ok(Some(self.translate_call("pow", node.children())?)),
            evalexpr::Operator::Eq => todo!(),
            evalexpr::Operator::Neq => todo!(),
            evalexpr::Operator::Gt => todo!(),
            evalexpr::Operator::Lt => todo!(),
            evalexpr::Operator::Geq => todo!(),
            evalexpr::Operator::Leq => todo!(),
            evalexpr::Operator::And => todo!(),
            evalexpr::Operator::Or => todo!(),
            evalexpr::Operator::Not => todo!(),
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
            evalexpr::Operator::AddAssign => todo!(),
            evalexpr::Operator::SubAssign => todo!(),
            evalexpr::Operator::MulAssign => todo!(),
            evalexpr::Operator::DivAssign => todo!(),
            evalexpr::Operator::ModAssign => todo!(),
            evalexpr::Operator::ExpAssign => todo!(),
            evalexpr::Operator::AndAssign => todo!(),
            evalexpr::Operator::OrAssign => todo!(),
            evalexpr::Operator::Tuple => todo!(),
            evalexpr::Operator::Chain => {
                let mut return_value = None;
                for ast in node.children() {
                    return_value = self.translate_operator(ast)?;
                }
                Ok(return_value)
            }
            evalexpr::Operator::Const { value } => match value {
                evalexpr::Value::String(_) => todo!(),
                evalexpr::Value::Float(value) => {
                    Ok(Some(self.builder.ins().f32const(*value as f32)))
                }
                evalexpr::Value::Int(value) => Ok(Some(self.builder.ins().f32const(*value as f32))),
                evalexpr::Value::Boolean(value) => {
                    Ok(Some(self.builder.ins().iconst(I64, *value as i64)))
                }
                evalexpr::Value::Tuple(_) => todo!(),
                evalexpr::Value::Empty => todo!(),
            },
            evalexpr::Operator::VariableIdentifierWrite { identifier: _ } => todo!(),
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

    fn translate_call(
        &mut self,
        identifier: &str,
        params: &[Node],
    ) -> Result<Value, EvalexprCompError> {
        let (func_ref, _) = self.declare_function(identifier)?;
        let params = params
            .iter()
            .map(|node| {
                let Some(value) = self.translate_operator(node)? else {
                    return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(
                        node.clone(),
                    ));
                };
                Ok(value)
            })
            .collect::<Result<Box<[Value]>, EvalexprCompError>>()?;
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
                todo!()
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
