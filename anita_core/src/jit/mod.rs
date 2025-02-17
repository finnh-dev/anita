use std::collections::HashMap;

use super::function_manager::FunctionManager;
use codegen::ir::FuncRef;
use cranelift::{
    codegen,
    prelude::{
        settings, AbiParam, Block, Configurable, EntityRef, FunctionBuilder,
        FunctionBuilderContext, InstBuilder, Signature, Variable,
    },
};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Module, ModuleError};
use frontend::{parser, Expr};
use peg::{error::ParseError, str::LineCol};
use translator::{ExprTranslator, TranslatorError};
use types::AnitaType;

pub mod compiled_function;
pub mod frontend;
mod translator;
pub mod types;

#[macro_export]
macro_rules! compile_expression {
    (@to_type $_:ident, $target:ty) => {$target};

    ($expression:expr, ($($parameter:ident),+) -> $target:ty) => {
        {
            use std::mem;
            use $crate::jit::{compiled_function::CompiledFunction, JIT};
            use $crate::function_manager::NoFunctions;

            let mut jit = JIT::<$target, NoFunctions>::default();
            match jit.compile($expression, &[$( stringify!($parameter) ),*]) {
                Ok(code_ptr) => {
                    let function_pointer = unsafe { mem::transmute::<*const u8, extern "C" fn($(compile_expression!(@to_type $parameter, $target)),+) -> $target>(code_ptr) };
                    let memory_region = jit.dissolve();
                    Ok(CompiledFunction::new(memory_region, function_pointer))
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
    };

    ($expression:expr, ($($parameter:ident),+) -> $target:ty, $functions:ty) => {
        {
            use std::mem;
            use $crate::jit::{compiled_function::CompiledFunction, JIT};

            let mut jit = JIT::<$target, $functions>::default();
            match jit.compile($expression, &[$( stringify!($parameter) ),*]) {
                Ok(code_ptr) => {
                    let function_pointer = unsafe { mem::transmute::<*const u8, extern "C" fn($(compile_expression!(@to_type $parameter, $target)),+) -> $target>(code_ptr) };
                    let memory_region = jit.dissolve();
                    Ok(CompiledFunction::new(memory_region, function_pointer))
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum JITError {
    TranslatorError(TranslatorError),
    ModuleError(ModuleError),
    ParseError(ParseError<LineCol>),
    UseOfUninitializedVariables(Box<[String]>),
    RootEvaluatesInNoValue,
}

impl From<TranslatorError> for JITError {
    fn from(value: TranslatorError) -> Self {
        Self::TranslatorError(value)
    }
}

impl From<ModuleError> for JITError {
    fn from(value: ModuleError) -> Self {
        Self::ModuleError(value)
    }
}

impl From<ParseError<LineCol>> for JITError {
    fn from(value: ParseError<LineCol>) -> Self {
        Self::ParseError(value)
    }
}

pub struct JIT<T: AnitaType, F: FunctionManager> {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Box<JITModule>,
    _function_manager: std::marker::PhantomData<F>,
    _type: std::marker::PhantomData<T>,
}

impl<T: AnitaType, F: FunctionManager> Default for JIT<T, F> {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder
            .set("use_colocated_libcalls", "false")
            .expect("Failed to set JIT flags");
        flag_builder
            .set("is_pic", "false")
            .expect("Failed to set JIT flags");
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .expect("Failed to finish ISA builder");
        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        builder.symbol("inbuilt_pow", T::inbuilt_pow as *const u8);
        for (ident, addr) in F::function_symbols() {
            builder.symbol(ident, addr);
        }
        let module = Box::new(JITModule::new(builder));
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
            _function_manager: std::marker::PhantomData,
            _type: std::marker::PhantomData,
        }
    }
}

impl<T: AnitaType, F: FunctionManager> JIT<T, F> {
    /// Drops self and returns an owned pointer to the memory region containing the compiled code.
    ///
    /// Can be used to manually manage the memory the validatity of the compiled function relies on.
    ///
    /// It is advised to use the provided [`compile_expression!`] macro instead.
    pub fn dissolve(self) -> Box<JITModule> {
        self.module
    }

    /// Compiles `expression` to a function of the `parameters` and returns the a pointer to the compiled code.
    ///
    /// The pointer remains valid until the module field of the JIT is deallocated.
    ///
    /// In order to manually manage the memory region [`JIT::dissolve`] can be used.
    ///
    /// It is advised to use the provided [`compile_expression!`] macro instead.
    pub fn compile<E: AsRef<str>>(
        &mut self,
        expression: E,
        parameters: &[&str],
    ) -> Result<*const u8, JITError> {
        let ast = parser::expression(expression.as_ref())?;

        self.translate(ast, parameters)?;

        let id = self.module.declare_function(
            "expression",
            cranelift_module::Linkage::Export,
            &self.ctx.func.signature,
        )?;

        self.module.define_function(id, &mut self.ctx)?;

        self.module.clear_context(&mut self.ctx);

        self.module
            .finalize_definitions()
            .expect("Failed to compile expression");
        Ok(self.module.get_finalized_function(id))
    }

    fn declare_inbuilt_functions(
        functions: &mut HashMap<String, (FuncRef, usize)>,
        builder: &mut FunctionBuilder,
        module: &mut JITModule,
    ) -> Result<(), ModuleError> {
        let inbuilt_pow_signature = Signature {
            params: vec![
                AbiParam::new(T::cranelift_repr()),
                AbiParam::new(T::cranelift_repr()),
            ],
            returns: vec![AbiParam::new(T::cranelift_repr())],
            call_conv: module.isa().default_call_conv(),
        };
        let func_id = module.declare_function(
            "inbuilt_pow",
            cranelift_module::Linkage::Import,
            &inbuilt_pow_signature,
        )?;
        let func = (module.declare_func_in_func(func_id, builder.func), 2);
        functions.insert("inbuilt_pow".to_owned(), func);
        Ok(())
    }

    fn translate(&mut self, root: Expr, params: &[&str]) -> Result<(), JITError> {
        for _name in params {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(T::cranelift_repr()));
        }

        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(T::cranelift_repr()));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let variables = Self::declare_variables(&mut builder, &root, params, entry_block)?;
        let mut functions = HashMap::default();

        Self::declare_inbuilt_functions(&mut functions, &mut builder, &mut self.module)?;

        let mut translator = ExprTranslator::<T, F> {
            builder: &mut builder,
            variables,
            functions,
            module: &mut self.module,
            _function_manager: std::marker::PhantomData,
            _type: std::marker::PhantomData,
        };

        let Some(return_value) = translator.translate(root)? else {
            return Err(JITError::RootEvaluatesInNoValue);
        };

        builder.ins().return_(&[return_value]);
        builder.finalize();

        Ok(())
    }

    fn declare_variables(
        builder: &mut FunctionBuilder,
        node: &Expr,
        params: &[&str],
        entry_block: Block,
    ) -> Result<HashMap<String, Variable>, JITError> {
        let mut variables = HashMap::new();
        let mut index = 0;

        let mut vars = node.variables();
        for (i, name) in params.iter().enumerate() {
            vars.set_defined(&name.to_string());
            let val = builder.block_params(entry_block)[i];
            let var = Self::declare_variable(builder, &mut variables, &mut index, name);
            builder.def_var(var, val);
        }
        let identifiers = match vars.initialized_identifiers() {
            Ok(i) => i,
            Err(uninitialized) => return Err(JITError::UseOfUninitializedVariables(uninitialized)),
        };

        for name in identifiers {
            let _ = Self::declare_variable(builder, &mut variables, &mut index, &name);
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
            builder.declare_var(var, T::cranelift_repr());
            *index += 1;
        }
        var
    }
}
