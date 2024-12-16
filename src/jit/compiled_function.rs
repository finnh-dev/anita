use std::{mem::ManuallyDrop, ops::Deref};

use cranelift_jit::JITModule;

struct FrozenJITModule {
    _module: ManuallyDrop<Box<JITModule>>,
}

impl From<Box<JITModule>> for FrozenJITModule {
    fn from(value: Box<JITModule>) -> Self {
        Self {
            _module: ManuallyDrop::new(value),
        }
    }
}

/// FrozenJITModule is only used to make sure the memory holding the compiled code is valid until the associated function pointer is dropped.
/// Therefore the JITModule is never modified after being frozen.
unsafe impl Sync for FrozenJITModule {}

impl Drop for FrozenJITModule {
    fn drop(&mut self) {
        let memory = unsafe {
            ManuallyDrop::take(&mut self._module)
        };
        unsafe {
            memory.free_memory();
        }
    }
}

pub struct CompiledFunction<F: Send + Sync> {
    _memory_region: FrozenJITModule,
    function_pointer: F,
}

impl<F: Send + Sync> CompiledFunction<F> {
    pub fn new(module: Box<JITModule>, function_pointer: F) -> CompiledFunction<F> {
        CompiledFunction {
            _memory_region: module.into(),
            function_pointer,
        }
    }
}

impl<F: Send + Sync> Deref for CompiledFunction<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.function_pointer
    }
}