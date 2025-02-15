#[test]
fn test_macro_expansion() {
    macrotest::expand("tests/expand/*test.rs");
}
