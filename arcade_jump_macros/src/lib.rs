/// Jump trajectory calculator
mod jump;

use proc_macro::TokenStream;

/// Compute jump parameters
#[proc_macro]
pub fn jump_parameters(input: TokenStream) -> TokenStream {
    jump::generate_calculator(input.into()).unwrap().into()
}
