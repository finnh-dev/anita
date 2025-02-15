pub trait FunctionManager {
    fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]>;
    fn function_signature(
        identifier: &str,
        calling_conventrion: cranelift::prelude::isa::CallConv,
    ) -> Option<cranelift::prelude::Signature>;
}

pub struct NoFunctions {}

impl FunctionManager for NoFunctions {
    fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]> {
        Box::default()
    }

    fn function_signature(
        _identifier: &str,
        _calling_conventrion: cranelift::prelude::isa::CallConv,
    ) -> Option<cranelift::prelude::Signature> {
        None
    }
}
