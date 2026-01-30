#[cfg(feature = "unstable")]
pub mod unstable_build_error {
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
pub mod stable_build_error {
    use core::fmt::Display;
    use proc_macro2::Span;
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
pub mod build_compile_error {
    use proc_macro2::TokenStream;
    use syn::DeriveInput;
    use syn::Error;
    pub fn build_error(err: Error) -> TokenStream {
        err.to_compile_error().into()
    }
}

