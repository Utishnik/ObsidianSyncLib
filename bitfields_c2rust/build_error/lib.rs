use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::DeriveInput;
use syn::Error;
use syn::parse_macro_input;
use syn::Meta;
use syn::Lit;

#[cfg(feature = "unstable")]
mod unstable_build_error {
    #![feature(proc_macro_totokens)]
    use core::fmt::Display;
    use proc_macro::ToTokens;
    use syn::DeriveInput;
    use syn::Error;

    pub fn build_error_feature<T, U>(tokens: T, message: U) -> Error
    where
        T: ToTokens + quote::ToTokens,
        U: Display,
    {
        Error::new_spanned(tokens, message)
    }
}

#[cfg(feature = "stable")]
mod stable_build_error {
    use core::fmt::Display;
    use proc_macro::Span;
    use syn::DeriveInput;
    use syn::Error;
    use syn::parse_macro_input;

    pub fn build_error_feature<U>(ast: &DeriveInput, message: U) -> Error
    where
        U: Display,
    {
        Error::new_spanned(ast, message)
    }

    pub fn build_position_error<U>(ast: &DeriveInput, message: U) -> Error
    where
        U: Display,
    {
        Error::new(ast.ident.span(), message)
    }

    pub fn build_span_error<U>(span: Span, message: U) -> Error
    where
        U: Display,
    {
        Error::new(span.into(), message)
    }
}

#[cfg(feature = "comp_err")]
mod build_compile_error {
    use proc_macro::TokenStream;
    use syn::DeriveInput;
    use syn::Error;
    pub fn build_error(ast: &DeriveInput, err: Error) -> TokenStream {
        err.to_compile_error().into()
    }
}

#[cfg(feature = "comp_err")]
#[proc_macro_attribute]
pub fn build_compiler_error(ast: TokenStream, err: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(ast);
    let err_convert: Result<syn::ItemMacro, Error> = syn::parse(err);
    if err_convert.is_err(){
        let err_convert_unwrap: Error = unsafe{err_convert.unwrap_err_unchecked()};
        let res: TokenStream = build_compile_error::build_error(&input, err_convert_unwrap);
        res
    }
    else{
        let empty: TokenStream = TokenStream::new();
        empty
    }
}

//нужно парсинг всех типов
#[cfg(feature = "comp_err")]
#[proc_macro_derive(err_feature, attributes(msg))]
pub fn build_error_feature(ast: TokenStream) -> TokenStream{
    use core::any::{Any, TypeId};
    let message: DeriveInput = parse_macro_input!(ast as DeriveInput);
    let attrl: Vec<Box<dyn Any>> = Vec::new();
    message.attrs
        .iter()
        .filter(|attr| attr.path().is_ident("msg"))
        .filter(|attr| {
            match attr.parse_args::<Meta>() {
                Ok(Meta::NameValue(name_value)) => {
                    // Используем name_value.value вместо name_value.lit
                    match &name_value.value {
                        syn::Expr::Lit(expr_lit) => {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                attrl.push(Box::new(lit_str.value()));
                        }
                        _ => ,
                    }
                }
                _ => {},
            }
        }
    });
    let input: DeriveInput = parse_macro_input!(ast);
    stable_build_error::build_position_error(&input, message);
}