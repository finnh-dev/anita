use std::{
    collections::{HashMap, HashSet},
    mem,
};

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Module, ModuleError};
use evalexpr::{build_operator_tree, EvalexprError, Node};
use itertools::Itertools;
use translator::ExprTranslator;
use types::F32;

use math::functions::{get_function_addr, get_function_symbols};
mod math;
mod translator;

#[macro_export]
macro_rules! compile_expression {
    (@to_f32 $_:ident) => {f32}; // TODO: investigate and potentially fix exposure of helper pattern outside this module

    ($expression:expr, ($($parameter:ident),+) -> f32) => {
        {
            use $crate::jit::{EvalexprFunction, EvalexprCompError, JIT};

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
    MalformedOperatorTree(Node),
    VariableNotFound(String),
}

impl EvalexprCompError {
    pub fn use_of_uninitialized_variables(uninitialized: &[&&str]) -> EvalexprCompError {
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

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
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
        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        for (ident, addr) in get_function_symbols() {
            builder.symbol(ident, addr);
        }
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

        let function_pointer = unsafe { mem::transmute::<*const u8, fn(I) -> O>(code_ptr) };

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
        let _ = get_function_addr("test_fn");
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
        let functions = HashMap::default();

        let mut translator = ExprTranslator {
            builder,
            variables,
            functions,
            module: &mut self.module,
        };

        let Some(return_value) = translator.translate_operator(&node)? else {
            return Err(EvalexprCompError::ExpressionEvaluatesToNoValue(node));
        };
        let return_value = translator.convert_value_type(F32, return_value)?;

        let (mut builder, _, _functions, _) = translator.deconstruct();

        // for (identifier, (_, _)) in functions {
        //     self.module.define_symbol()
        // }
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
            &uninitialized,
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
            &uninitialized,
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
        builder.declare_var(var, F32); // TODO: allow different variable types or make default behavior
        *index += 1;
    }
    var
}
