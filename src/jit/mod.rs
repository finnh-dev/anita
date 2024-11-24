use std::{
    collections::{HashMap, HashSet},
    mem,
};

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Module, ModuleError};
use evalexpr::{build_operator_tree, EvalexprError, Node, Operator};
use itertools::Itertools;
use types::{F32, I64};

#[macro_export]
macro_rules! compile_expression {
    (@to_f32 $_:ident) => {f32}; // TODO: investigate and potentially fix exposure of helper pattern outside this module

    ($expression:expr, ($($parameter:ident),+) -> f32) => {
        {
            use jit::{EvalexprFunction, EvalexprCompError, JIT};

            let jit = JIT::default();
            #[allow(unused_parens)] // necessary due to https://github.com/rust-lang/rust/issues/73068
            let function: Result<EvalexprFunction<($(compile_expression!(@to_f32 $parameter)),+), f32>, EvalexprCompError> = jit.compile($expression, &[$( stringify!($parameter) ),*]);
            function
        }
    };
}

#[derive(Debug)]
pub enum EvalexprCompError {
    EvalexprError(EvalexprError),
    CompilerError(ModuleError),
    UseOfUninitializedVariables(Box<[String]>),
    UnsupportedTypeConversion {
        target_type: Type,
        source_type: Type,
    },
    ExpressionEvaluatesToNoValue(Node),
    UseOfUnsupportedType(Type),
    MalformedOperatorTree,
}

impl EvalexprCompError {
    pub fn use_of_uninitialized_variables(uninitialized: Box<[&&str]>) -> EvalexprCompError {
        EvalexprCompError::UseOfUninitializedVariables(
            uninitialized.iter().map(|x| x.to_string()).collect(),
        )
    }
}

impl From<EvalexprError> for EvalexprCompError {
    fn from(value: EvalexprError) -> Self {
        EvalexprCompError::EvalexprError(value)
    }
}

impl From<ModuleError> for EvalexprCompError {
    fn from(value: ModuleError) -> Self {
        EvalexprCompError::CompilerError(value)
    }
}

// TODO: make generic/implement constructor macro for different function signatures
#[derive(Debug)]
pub struct EvalexprFunction<I, O> {
    #[allow(unused)]
    memory_region: Box<dyn std::any::Any>,
    function_pointer: fn(I) -> O,
}

impl<I, O> EvalexprFunction<I, O> {
    pub fn execute(&self, x: I) -> O {
        (self.function_pointer)(x)
    }
}

/// The basic JIT class.
pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
        }
    }
}

impl JIT {
    fn finalize<I, O>(self, func_id: FuncId) -> EvalexprFunction<I, O> {
        let code_ptr = self.module.get_finalized_function(func_id);

        let function_pointer = unsafe { mem::transmute::<_, fn(I) -> O>(code_ptr) };

        let memory_region = Box::new(self.module);
        EvalexprFunction {
            memory_region,
            function_pointer,
        }
    }

    pub fn compile<E: AsRef<str>, I, O>(
        mut self,
        expression: E,
        params: &[&str],
    ) -> Result<EvalexprFunction<I, O>, EvalexprCompError> {
        let ast = build_operator_tree(expression.as_ref())?;

        self.translate(ast, params)?;

        let id = self.module.declare_function(
            "waveshaper",
            cranelift_module::Linkage::Export,
            &self.ctx.func.signature,
        )?;

        self.module.define_function(id, &mut self.ctx)?;

        self.module.clear_context(&mut self.ctx);

        self.module
            .finalize_definitions()
            .expect("Failed to compile expression");
        Ok(self.finalize(id))
    }

    fn translate(&mut self, node: Node, params: &[&str]) -> Result<(), EvalexprCompError> {
        for _name in params {
            self.ctx.func.signature.params.push(AbiParam::new(F32));
        }

        self.ctx.func.signature.returns.push(AbiParam::new(F32)); // Always returns f32

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let variables = declare_variables(&mut builder, &node, params, entry_block)?;

        let mut translator = ExprTranslator { builder, variables };

        let Some(return_value) = translator.translate_operator(&node)? else {
            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(node));
        };
        let mut builder = translator.builder;

        builder.ins().return_(&[return_value]);
        builder.finalize();

        Ok(())
    }
}

fn declare_variables(
    builder: &mut FunctionBuilder,
    node: &Node,
    params: &[&str],
    entry_block: Block,
) -> Result<HashMap<String, Variable>, EvalexprCompError> {
    let mut variables = HashMap::new();
    let mut index = 0;

    let assignments = node
        .iter_write_variable_identifiers()
        .collect::<Box<[&str]>>();
    let initialized = params
        .iter()
        .merge(assignments.iter())
        .collect::<HashSet<&&str>>();
    let identifiers: Box<[&str]> = node.iter_variable_identifiers().unique().collect();
    let uninitialized: Box<[&&str]> = identifiers
        .iter()
        .filter(|x| !initialized.contains(x))
        .collect();
    if uninitialized.len() > 0 {
        return Err(EvalexprCompError::use_of_uninitialized_variables(
            uninitialized,
        ));
    }

    let assignments = node
        .iter_write_variable_identifiers()
        .collect::<Box<[&str]>>();
    let initialized = params
        .iter()
        .merge(assignments.iter())
        .collect::<HashSet<&&str>>();
    let identifiers: Box<[&str]> = node.iter_variable_identifiers().unique().collect();
    let uninitialized: Box<[&&str]> = identifiers
        .iter()
        .filter(|x| !initialized.contains(x))
        .collect();
    if uninitialized.len() > 0 {
        return Err(EvalexprCompError::use_of_uninitialized_variables(
            uninitialized,
        ));
    }

    for (i, name) in params.iter().enumerate() {
        let val = builder.block_params(entry_block)[i];
        let var = declare_variable(builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }

    for name in identifiers {
        let _ = declare_variable(builder, &mut variables, &mut index, name);
    }

    Ok(variables)
}

fn declare_variable(
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, F32);
        *index += 1;
    }
    var
}

struct ExprTranslator<'a> {
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
}

impl<'a> ExprTranslator<'a> {
    pub fn translate_operator(&mut self, node: &Node) -> Result<Option<Value>, EvalexprCompError> {
        match node.operator() {
            evalexpr::Operator::RootNode => {
                let children = node.children();
                if children.len() > 1 {
                    return Err(EvalexprCompError::MalformedOperatorTree);
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
                let (_lhs, _rhs) = self.binary_operation(node)?;
                todo!()
            }
            evalexpr::Operator::Exp => {
                let (_lhs, _rhs) = self.binary_operation(node)?;
                todo!()
            }
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

                let Operator::VariableIdentifierWrite { identifier } = target.operator() else {
                    return Err(EvalexprCompError::MalformedOperatorTree);
                };
                let variable = *self
                    .variables
                    .get(identifier)
                    .expect(&format!("Variable {} does not exist", identifier));

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
                println!("chain return: {:?}", return_value);
                Ok(return_value)
            }
            evalexpr::Operator::Const { value } => match value {
                evalexpr::Value::String(_) => todo!(),
                evalexpr::Value::Float(value) => {
                    Ok(Some(self.builder.ins().f32const(*value as f32)))
                }
                evalexpr::Value::Int(value) => {
                    Ok(Some(self.builder.ins().iconst(I64, *value)))
                },
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
                    .expect(&format!("Variable {} does not exist", identifier));
                Ok(Some(self.builder.use_var(*variable)))
            }
            evalexpr::Operator::FunctionIdentifier { identifier: _ } => todo!(),
        }
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

    fn convert_value_type(
        &mut self,
        target_type: Type,
        value: Value,
    ) -> Result<Value, EvalexprCompError> {
        match (target_type, self.check_value_type(value)) {
            (target_type, source_type) if target_type == source_type => Ok(value),
            (target_type, source_type)
                if target_type.is_int()
                    && source_type.is_int()
                    && target_type.bits() > source_type.bits() =>
            {
                Ok(self.builder.ins().sextend(target_type, value))
            }
            (target_type, source_type)
                if target_type.is_int()
                    && source_type.is_int()
                    && target_type.bits() < source_type.bits() =>
            {
                Ok(self.builder.ins().ireduce(target_type, value))
            }
            (target_type, source_type) if target_type.is_int() && source_type.is_float() => {
                Ok(self.builder.ins().fcvt_to_sint(target_type, value))
            }
            (target_type, source_type) if target_type.is_float() && source_type.is_int() => {
                Ok(self.builder.ins().fcvt_from_sint(target_type, value))
            }
            (target_type, source_type)
                if target_type.is_float()
                    && source_type.is_float()
                    && target_type.bits() > source_type.bits() =>
            {
                Ok(self.builder.ins().fpromote(target_type, value))
            }
            (target_type, source_type)
                if target_type.is_float()
                    && source_type.is_float()
                    && target_type.bits() > source_type.bits() =>
            {
                Ok(self.builder.ins().fdemote(target_type, value))
            }
            (target_type, source_type) => Err(EvalexprCompError::UnsupportedTypeConversion {
                target_type,
                source_type,
            }),
        }
    }
}
