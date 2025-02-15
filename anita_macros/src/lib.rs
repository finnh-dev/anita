use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Block, Ident, ItemFn, LitStr, PatType, ReturnType, Token, Type,
};

#[derive(Debug)]
struct ImplBlock {
    attributes: Vec<Attribute>,
    ident: Ident,
    functions: Vec<Function>,
}

impl Parse for ImplBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let _impl = input.parse::<syn::Token![impl]>()?;
        let ident = input.parse()?;
        let inner;
        let _bracket = braced!(inner in input);

        Ok(Self {
            attributes,
            ident,
            functions: parse_zero_or_more(&inner),
        })
    }
}

impl ToTokens for ImplBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attributes = &self.attributes;
        let ident = &self.ident;
        let functions = &self.functions;
        tokens.extend(quote! {
            #(#attributes)*
            impl #ident {
                #(pub extern "C" #functions)*
            }
        });
    }
}

struct NameAttribute {
    alias: LitStr,
}

mod keyword {
    syn::custom_keyword!(name);
}

impl Parse for NameAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _pound = input.parse::<Token![#]>()?;
        let attribute_inner;
        let _bracket = bracketed!(attribute_inner in input);
        let _name_attribute = attribute_inner.parse::<keyword::name>()?;
        let _eq = attribute_inner.parse::<Token![=]>()?;
        let alias = attribute_inner.parse()?;
        Ok(Self { alias })
    }
}

#[derive(Debug)]
struct Function {
    fn_item: ItemFn,
    alias: Option<LitStr>,
    ident: Ident,
    arguments: Vec<PatType>,
    return_type: ReturnType,
}

impl Parse for Function {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let forked = input.fork();
        let _attrs = forked.call(Attribute::parse_outer);
        let fn_item = forked.parse()?;

        let name_attribute: Option<NameAttribute> = input.parse().ok();
        let _fn = input.parse::<Token![fn]>()?;
        let ident = input.parse()?;
        let arguments_block;
        let _par = parenthesized!(arguments_block in input);
        let arguments = parse_all(&arguments_block)?;
        let return_type = input.parse()?;
        let _body = input.parse::<Block>()?;

        Ok(Self {
            fn_item,
            alias: name_attribute.map(|a| a.alias),
            ident,
            arguments,
            return_type,
        })
    }
}

impl ToTokens for Function {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.fn_item.to_tokens(tokens);
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

fn parse_all<T: Parse>(input: ParseStream) -> syn::Result<Vec<T>> {
    let mut result = Vec::new();
    while !input.is_empty() {
        result.push(input.parse()?);
        let _comma = input.parse::<Option<Token![,]>>()?;
    }
    Ok(result)
}

struct FunctionSignature {
    crate_name: proc_macro2::TokenStream,
    ident: LitStr,
    arguments: Vec<Type>,
    return_type: Type,
}

impl FunctionSignature {
    fn try_from_function(
        value: &Function,
        crate_name: proc_macro2::TokenStream,
    ) -> syn::Result<Self> {
        let arguments = value.arguments.iter().map(|arg| *arg.ty.clone()).collect();
        let ReturnType::Type(_, return_type) = value.return_type.clone() else {
            return Err(syn::Error::new_spanned(
                value.return_type.clone(),
                "Functions must return a value",
            ));
        };
        let ident = match value.alias {
            Some(ref alias) => alias.clone(),
            None => LitStr::new(&value.ident.to_string(), value.ident.span()),
        };
        Ok(Self {
            crate_name,
            ident,
            arguments,
            return_type: *return_type,
        })
    }
}

impl ToTokens for FunctionSignature {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_name = &self.crate_name;
        let ident = &self.ident;
        let arguments = &self.arguments;
        let return_type = &self.return_type;
        tokens.extend(quote! {
            #ident => Some(#crate_name::cranelift::prelude::Signature {
                params: std::vec![#(#crate_name::cranelift::prelude::AbiParam::new(#arguments::cranelift_repr()),)*],
                returns: std::vec![#crate_name::cranelift::prelude::AbiParam::new(#return_type::cranelift_repr())],
                call_conv: calling_convention,
            })
        });
    }
}
struct FunctionSymbol {
    ident: Ident,
    alias: LitStr,
}

impl From<&Function> for FunctionSymbol {
    fn from(value: &Function) -> Self {
        let alias = match value.alias {
            Some(ref alias) => alias.clone(),
            None => LitStr::new(&value.ident.to_string(), value.ident.span()),
        };
        Self {
            ident: value.ident.clone(),
            alias,
        }
    }
}

impl ToTokens for FunctionSymbol {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let alias = &self.alias;
        let ident = &self.ident;
        tokens.extend(quote! {
            (#alias, Self::#ident as *const u8)
        });
    }
}

struct FunctionManager {
    crate_name: proc_macro2::TokenStream,
    ident: Ident,
    function_signatures: Vec<FunctionSignature>,
    function_symbols: Vec<FunctionSymbol>,
}

impl FunctionManager {
    fn new(impl_block: &ImplBlock, crate_name: proc_macro2::TokenStream) -> syn::Result<Self> {
        let ident = impl_block.ident.clone();
        let function_signatures = impl_block
            .functions
            .iter()
            .map(|f| FunctionSignature::try_from_function(f, crate_name.clone()))
            .collect::<syn::Result<Vec<FunctionSignature>>>()?;
        let function_symbols = impl_block.functions.iter().map(|f| f.into()).collect();
        Ok(Self {
            crate_name,
            ident,
            function_signatures,
            function_symbols,
        })
    }
}

impl ToTokens for FunctionManager {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let function_symbols = &self.function_symbols;
        let function_signatures = &self.function_signatures;
        let crate_name = &self.crate_name;
        tokens.extend(quote! {
            impl #crate_name::FunctionManager for #ident {
                fn function_symbols() -> std::boxed::Box<[(&'static str, *const u8)]> {
                    std::boxed::Box::new([#(#function_symbols,)*])
                }

                fn function_signature(
                    identifier: &str,
                    calling_convention: #crate_name::cranelift::prelude::isa::CallConv,
                ) -> Option<#crate_name::cranelift::prelude::Signature> {
                    match identifier {
                        #(#function_signatures,)*
                        _ => None
                    }
                }
            }
        });
    }
}

#[proc_macro_attribute]
pub fn function_manager(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ImplBlock);

    let crate_name = match crate_name("anita").unwrap_or(proc_macro_crate::FoundCrate::Itself) {
        proc_macro_crate::FoundCrate::Itself => quote! { crate },
        proc_macro_crate::FoundCrate::Name(_) => quote! { anita },
    };
    let function_manager = match FunctionManager::new(&impl_block, crate_name.clone()) {
        Ok(function_manager) => function_manager,
        Err(e) => return e.into_compile_error().into(),
    };

    let result = quote! {
        #impl_block

        #function_manager
    };
    
    result.into()
}
