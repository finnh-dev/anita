struct TestFunctionManager;

#[cfg(not(feature="whatever"))]
#[internal_macros::function_manager]
impl TestFunctionManager {}