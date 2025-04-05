/// Input and output parameters
mod parameter;

/// Select the function
mod select;

/// How to read a statement
mod statement;

use crate::utils::ParseError;
use parameter::{Parameter, ParameterInput, ParameterOutput};
use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote;
use statement::Statement;
use thiserror::Error;

/// Parse token stream and generate instruction to compute variables
pub(crate) fn generate_calculator(tokens: TokenStream) -> Result<TokenStream, CompError> {
    // read the stream of tokens
    let mut iter = tokens.into_iter();

    // Parse the token stream into a statement
    Statement::parse(&mut iter)?.to_tokens()
}

/// Read a sequence of tokens to get the expected type
pub(crate) trait ParseTokens: Sized {
    fn parse(iter: &mut IntoIter) -> Result<Self, CompError>;
}

/// The type of errors encountered when analyzing a statement
#[derive(Error, Debug)]
pub(crate) enum CompError {
    /// We expected a parameter but it was not found
    #[error("Missing parameter")]
    Missing,

    /// The parameter type is not one of the four supported types
    #[error("Invalid parameter type {0}")]
    InvalidType(TokenTree),

    /// The parameter expression is not a valid expression
    #[error("Invalid parameter expression {0}")]
    InvalidExpr(TokenTree),

    /// Missing arrow symbol
    #[error("Missing arrow symbol")]
    MissingArrow,

    /// Invalid end of statement
    #[error("Invalid end of statement")]
    InvalidEnd(TokenTree),

    /// The combination of parameters is invalid
    #[error("Invalid parameter combination {input1} {input2} => {output}")]
    InvalidCombination {
        input1: ParameterInput,
        input2: ParameterInput,
        output: ParameterOutput,
    },

    /// Parsing error
    #[error("Parsing error {0}")]
    ParsingError(#[from] ParseError),
}

impl From<CompError> for syn::Error {
    fn from(value: CompError) -> Self {
        match value {
            CompError::Missing => syn::Error::new_spanned(TokenStream::new(), "Missing parameter"),
            CompError::InvalidType(tt) => syn::Error::new_spanned(tt, "Invalid parameter type"),
            CompError::InvalidExpr(tt) => {
                syn::Error::new_spanned(tt, "Invalid parameter expression")
            }
            CompError::MissingArrow => {
                syn::Error::new_spanned(TokenStream::new(), "Missing arrow symbol")
            }
            CompError::InvalidEnd(tt) => syn::Error::new_spanned(tt, "Invalid end of statement"),
            CompError::InvalidCombination {
                input1,
                input2,
                output,
            } => {
                let in1 = input1.get_type();
                let in2 = input2.get_type();
                let out = output.get_type();
                let tokens = quote![ #in1, #in2 => #out ];
                syn::Error::new_spanned(tokens, "Invalid parameter combination")
            }
            CompError::ParsingError(err) => syn::Error::from(err),
        }
    }
}
