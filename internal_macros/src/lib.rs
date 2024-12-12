use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use quote::quote;
use syn::{
    parse_macro_input, Expr, FnArg, Ident, ImplItemFn,
    ItemImpl, Lit, Meta, PatType, Path, PathSegment, ReturnType, Type, TypePath,
};

#[derive(Debug)]
struct FnSignature {
    name: Ident,
    identifier: Ident,
    params: Vec<String>,
    return_type: String,
}

#[proc_macro_attribute]
pub fn function_manager(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);
    // eprintln!("impl_block: {:#?}", impl_block);

    let trait_name = match crate_name("anita").unwrap_or(proc_macro_crate::FoundCrate::Itself) {
        proc_macro_crate::FoundCrate::Itself => quote! { crate::function_manager::FunctionManager },
        proc_macro_crate::FoundCrate::Name(_) => quote! { anita::function_manager::FunctionManager },
    };


    let functions = match impl_block
        .items
        .iter()
        .map(|item| match item {
            syn::ImplItem::Fn(function) => Ok(function),
            item => Err(syn::Error::new_spanned(item, "expected `fn ...`")
                .to_compile_error()
                .into()),
        })
        .collect::<Result<Vec<&ImplItemFn>, TokenStream>>()
    {
        Ok(functions) => functions,
        Err(e) => return e,
    };

    let extern_c_functions = functions.iter().map(|function| {
        let mut function = (**function).clone();
        function.attrs.clear();
        quote! {
            #[no_mangle]
            pub extern "C" #function
        }
    });

    let signatures: Vec<FnSignature> = functions
        .iter()
        .map(|function| extract_signature(function))
        .collect();

    let match_func_addr = signatures.iter().map(|sig| {
        let name = &sig.name;
        let ident = &sig.identifier;
        quote! {
            stringify!(#name) => Some(Self::#ident as *const u8)
        }
    });

    let func_symbols = signatures.iter().map(|sig| {
        let name = &sig.name;
        let ident = &sig.identifier;
        quote! {
            (stringify!(#name), Self::#ident as *const u8)
        }
    });

    let match_signatures = signatures.iter().map(|sig| {
        let name = &sig.name;
        let params = sig
            .params
            .iter()
            .map(|ty| to_cranelift_parameter(ty.as_str()));
        let return_type = to_cranelift_parameter(sig.return_type.as_str());
        quote! {
            stringify!(#name) => Some(cranelift::prelude::Signature {
                params: std::vec![#(#params,)*],
                returns: std::vec![#return_type],
                call_conv: calling_convention,
            })
        }
    });

    let impl_type = impl_block.self_ty;

    quote! {
        impl #impl_type {
            #(#extern_c_functions)*
        }

        impl #trait_name for #impl_type {
            fn function_address(identifier: &str) -> Option<*const u8> {
                match identifier {
                    #(#match_func_addr,)*
                    _ => None
                }
            }
            fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]> {
                std::boxed::Box::new([#(#func_symbols,)*])
            }
            fn function_signature(
                identifier: &str,
                calling_convention: cranelift::prelude::isa::CallConv,
            ) -> Option<cranelift::prelude::Signature> {
                match identifier {
                    #(#match_signatures,)*
                    _ => None
                }
            }
        }
    }
    .into()
}

// #[proc_macro]
// pub fn link_cranelift(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as File);

//     let functions = match ast
//         .items
//         .iter()
//         .map(|item| match item {
//             syn::Item::Fn(function) => Ok(function),
//             item => Err(syn::Error::new_spanned(item, "expected `fn ...`")
//                 .to_compile_error()
//                 .into()),
//         })
//         .collect::<Result<Vec<&ItemFn>, TokenStream>>()
//     {
//         Ok(functions) => functions,
//         Err(e) => return e,
//     };

//     let extern_c_functions = functions.iter().map(|function| {
//         let mut function = (**function).clone();
//         function.attrs.clear();
//         quote! {
//             #[no_mangle]
//             pub extern "C" #function
//         }
//     });

//     let signatures: Vec<FnSignature> = functions
//         .iter()
//         .map(|function| extract_signature(function))
//         .collect();

//     let match_func_addr = signatures.iter().map(|sig| {
//         let name = &sig.name;
//         let ident = &sig.identifier;
//         quote! {
//             stringify!(#name) => Some(#ident as *const u8)
//         }
//     });

//     let func_symbols = signatures.iter().map(|sig| {
//         let name = &sig.name;
//         let ident = &sig.identifier;
//         quote! {
//             (stringify!(#name), #ident as *const u8)
//         }
//     });

//     let match_signatures = signatures.iter().map(|sig| {
//         let name = &sig.name;
//         let params = sig
//             .params
//             .iter()
//             .map(|ty| to_cranelift_parameter(ty.as_str()));
//         let return_type = to_cranelift_parameter(sig.return_type.as_str());
//         quote! {
//             stringify!(#name) => Some(cranelift::prelude::Signature {
//                 params: std::vec![#(#params,)*],
//                 returns: std::vec![#return_type],
//                 call_conv,
//             })
//         }
//     });

//     quote! {
//         #(#extern_c_functions)*

//         pub(crate) fn get_function_addr(identifier: &str) -> Option<*const u8> {
//             match identifier {
//                 #(#match_func_addr,)*
//                 _ => None
//             }
//         }

//         pub(crate) fn get_function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]> {
//             std::boxed::Box::new([#(#func_symbols,)*])
//         }

//         pub(crate) fn get_function_signature(identifier: &str, call_conv: cranelift::prelude::isa::CallConv) -> Option<cranelift::prelude::Signature> {
//             match identifier {
//                 #(#match_signatures,)*
//                 _ => None
//             }
//         }
//     }
//     .into()
// }

fn get_name_attribute(function: &ImplItemFn) -> Option<Ident> {
    for attribute in &function.attrs {
        match &attribute.meta {
            Meta::NameValue(meta_name_value) => {
                if !meta_name_value.path.is_ident("name") {
                    continue;
                }
                let Expr::Lit(literal) = &meta_name_value.value else {
                    continue;
                };
                let Lit::Str(string_literal) = &literal.lit else {
                    continue;
                };
                let identifier = Ident::new(&string_literal.value(), string_literal.span());
                return Some(identifier);
            }
            _ => continue,
        }
    }
    None
}

fn extract_signature(function: &ImplItemFn) -> FnSignature {
    let identifier = function.sig.ident.clone();
    let name = get_name_attribute(function).unwrap_or(identifier.clone());
    let mut params = Vec::new();
    for param in &function.sig.inputs {
        let FnArg::Typed(PatType { ty, .. }) = param else {
            panic!("unexpected function parameter: {:?}", param);
        };
        let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = *ty.clone()
        else {
            panic!("unexpected parameter type: {:?}", ty);
        };
        let Some(PathSegment { ident, .. }) = segments.last() else {
            panic!("malformed path in paramter: {:?}", segments);
        };
        params.push(format!("{}", ident));
    }

    let ReturnType::Type(_, ty) = &function.sig.output else {
        panic!("unexpected return value: {:?}", function.sig.output);
    };

    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = *ty.clone()
    else {
        panic!("unexpected return type: {:?}", ty);
    };

    let Some(PathSegment { ident, .. }) = segments.last() else {
        panic!("malformed path in return type: {:?}", segments);
    };

    let return_type = format!("{}", ident);

    FnSignature {
        name,
        identifier,
        params,
        return_type,
    }
}

fn to_cranelift_type(ty: &str) -> proc_macro2::TokenStream {
    match ty {
        "f32" => quote! {cranelift::prelude::types::F32},
        _ => panic!("use of unsupported parameter type: {}", ty),
    }
}

fn to_cranelift_parameter(parameter_type: &str) -> proc_macro2::TokenStream {
    let parameter_type = to_cranelift_type(parameter_type);
    quote! {cranelift::prelude::AbiParam::new(#parameter_type)}
}
