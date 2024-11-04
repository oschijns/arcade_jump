mod solver;

use proc_macro::TokenStream;

/// Compute jump parameters
#[proc_macro]
pub fn solve_jump_parameter(input: TokenStream) -> TokenStream {
    solver::generate_solver(input.into()).unwrap().into()
}
