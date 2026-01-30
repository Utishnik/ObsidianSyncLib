use build_error_general_api::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::DeriveInput;
use syn::Error;
use syn::Lit;
use syn::Meta;
use syn::parse_macro_input;

macro_rules! bug_match {
    () => {
        panic!("pattern match err")
    };
}

#[cfg(feature = "comp_err")]
#[proc_macro_derive(err_pos, attributes(msg))]
pub fn build_pos_error(ast: TokenStream) -> TokenStream {
    use core::any::{Any, TypeId};
    let clone_ast: TokenStream = ast.clone();
    let message: DeriveInput = parse_macro_input!(clone_ast as DeriveInput);
    let mut attrl: Vec<Box<dyn Any>> = Vec::new();
    let _ = message
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("msg"))
        .map(|attr| {
            match attr.parse_args::<Meta>() {
                Ok(Meta::NameValue(name_value)) => {
                    // Используем name_value.value вместо name_value.lit
                    match &name_value.value {
                        syn::Expr::Lit(expr_lit) => {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                attrl.push(Box::new(lit_str.value()));
                            } else {
                            }
                        }
                        _ => {
                            dbg!("please convert to string");
                        }
                    }
                }
                _ => {
                    bug_match!()
                }
            }
        });
    let input: DeriveInput = parse_macro_input!(ast);
    let first_val: Option<&Box<dyn Any>> = attrl.first();
    if first_val.is_none() {
        panic!("build_error_feature not find attr!");
    }
    let first_val_unwrap: &Box<dyn Any> = unsafe { first_val.unwrap_unchecked() };
    let first_val_downcast: Option<&String> = first_val_unwrap.downcast_ref::<String>();
    if first_val_downcast.is_none() {
        dbg!("please convert to string");
        let empty: TokenStream = TokenStream::new();
        return empty;
    }
    let first_val_downcast_unwrap: &String = unsafe { first_val_downcast.unwrap_unchecked() };
    let res_err: Error =
        stable_build_error::build_position_error(&input, first_val_downcast_unwrap);
    build_error_general_api::build_compile_error::build_error(res_err).into()
}

#[cfg(feature = "comp_err")]
#[proc_macro_derive(err_feature, attributes(msg))]
pub fn build_error_feature(ast: TokenStream) -> TokenStream {
    use core::any::{Any, TypeId};
    let clone_ast: TokenStream = ast.clone();
    let message: DeriveInput = parse_macro_input!(clone_ast as DeriveInput);
    let mut attrl: Vec<Box<dyn Any>> = Vec::new();
    let _ = message
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("msg"))
        .map(|attr| {
            match attr.parse_args::<Meta>() {
                Ok(Meta::NameValue(name_value)) => {
                    // Используем name_value.value вместо name_value.lit
                    match &name_value.value {
                        syn::Expr::Lit(expr_lit) => {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                attrl.push(Box::new(lit_str.value()));
                            } else {
                            }
                        }
                        _ => {
                            dbg!("please convert to string");
                        }
                    }
                }
                _ => {
                    bug_match!()
                }
            }
        });
    let input: DeriveInput = parse_macro_input!(ast);
    let first_val: Option<&Box<dyn Any>> = attrl.first();
    if first_val.is_none() {
        panic!("build_error_feature not find attr!");
    }
    let first_val_unwrap: &Box<dyn Any> = unsafe { first_val.unwrap_unchecked() };
    let first_val_downcast: Option<&String> = first_val_unwrap.downcast_ref::<String>();
    if first_val_downcast.is_none() {
        dbg!("please convert to string");
        let empty: TokenStream = TokenStream::new();
        return empty;
    }
    let first_val_downcast_unwrap: &String = unsafe { first_val_downcast.unwrap_unchecked() };
    let res_err: Error = stable_build_error::build_error_feature(&input, first_val_downcast_unwrap);
    build_error_general_api::build_compile_error::build_error(res_err).into()
}
