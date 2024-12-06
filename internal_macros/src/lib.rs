use syn::parse_macro_input;

#[proc_macro]
pub fn link_cranelift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as proc_macro2::TokenStream);
    println!("ast: {:#?}", ast);
    todo!();
}