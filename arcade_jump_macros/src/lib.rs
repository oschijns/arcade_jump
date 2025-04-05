#![no_std]

/// Jump trajectory calculator
mod jump;

/// Common tools for parsing tokens
mod utils;

extern crate alloc;
use proc_macro::TokenStream;
use syn::Error;

/// Compute jump parameters
#[proc_macro]
pub fn compute(input: TokenStream) -> TokenStream {
    match jump::generate_calculator(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => Error::from(err).to_compile_error().into(),
    }
}
