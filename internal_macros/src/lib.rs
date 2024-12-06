use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, File, FnArg, Ident, ItemFn, PatType, Path, PathSegment, ReturnType, Type,
    TypePath,
};

#[derive(Debug)]
struct FnSignature {
    identifier: String,
    params: Vec<String>,
    return_type: String,
}

#[proc_macro]
pub fn link_cranelift(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as File);
    // eprintln!("ast: {:#?}", ast);

    let functions: Vec<&ItemFn> = ast
        .items
        .iter()
        .map(|item| match item {
            syn::Item::Fn(function) => function,
            item => panic!("unexpected item: {:?}", item),
        })
        .collect();

    let extern_c_functions = functions.iter().map(|function| {
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
        let ident = Ident::new(&sig.identifier, Span::call_site());
        quote! {
            stringify!(#ident) => Some(#ident as *const u8)
        }
    });

    let match_signatures = signatures.iter().map(|sig| {
        let ident = Ident::new(&sig.identifier, Span::call_site());
        let params = sig
            .params
            .iter()
            .map(|ty| to_cranelift_parameter(ty.as_str()));
        let return_type = to_cranelift_parameter(sig.return_type.as_str());
        quote! {
            #ident => Some(cranelift::prelude::Signature {
                params: std::vec![#(#params,)*],
                returns: std::vec![#return_type],
                call_conv: cranelift::prelude::isa::CallConv::SystemV,
            })
        } // TODO: Figure out if this is the right calling convention and if its supposed to be hardcoded
    });

    // eprintln!("Functions: {:?}", signatures);

    quote! {
        #(#extern_c_functions)*

        pub(crate) fn get_function_addr(identifier: &str) -> Option<*const u8> {
            match identifier {
                #(#match_func_addr,)*
                _ => None
            }
        }

        pub(crate) fn get_function_signature(identifier: &str) -> Option<cranelift::prelude::Signature> {
            match identifier {
                #(#match_signatures,)*
                _ => None
            }
        }
    }
    .into()
}

fn extract_signature(function: &ItemFn) -> FnSignature {
    let identifier = format!("{}", function.sig.ident);
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
