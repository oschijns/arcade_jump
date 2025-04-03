/// Which floating type number to use
mod config;

/// Input and output parameters
mod parameter;

/// Select the function
mod select;

/// How to read a statement
mod statement;

use config::FloatType;
use parameter::{ParameterInput, ParameterOutput};
use proc_macro2::{Ident, Punct, TokenStream, TokenTree, token_stream::IntoIter};
use statement::Statement;

/// Parse token stream and generate instruction to compute variables
pub(crate) fn generate_calculator(tokens: TokenStream) -> Result<TokenStream, SolveError> {
    // read the stream of tokens
    let mut iter = tokens.into_iter();

    // Collect the statements
    let mut statements = Vec::<Statement>::new();

    // The first statement is the float type to use
    let float = FloatType::parse(&mut iter)?;

    // Read all the statements
    loop {
        match Statement::parse(&mut iter) {
            Ok(stmt) => {
                statements.push(stmt);
            }
            Err(SolveError::End) => {
                break;
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    // Generate the statements
    let mut output = TokenStream::new();
    for (index, stmt) in statements.iter().enumerate() {
        output.extend(stmt.to_tokens(&float, index)?);
    }

    Ok(output)
}

/// The type of errors encountered when parsing statements
#[repr(u32)]
#[derive(Debug)]
pub(crate) enum SolveError {
    /// End of the stream of tokens
    End,

    /// Error in the syntax
    Syntax(TokenTree),

    /// Error on the sequence of parameters
    Parameter {
        input1: ParameterInput,
        input2: ParameterInput,
        output: ParameterOutput,
    },
}

/// Read a sequence of tokens to get the expected type
pub(crate) trait ParseTokens: Sized {
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError>;
}

/// Get the next token and expect it to be a punctuation
fn get_punct(iter: &mut IntoIter) -> Result<Punct, SolveError> {
    if let Some(token) = iter.next() {
        match token {
            TokenTree::Punct(punct) => Ok(punct),
            _ => Err(SolveError::Syntax(token)),
        }
    } else {
        Err(SolveError::End)
    }
}

/// Check if the next token is the specified punctuation
fn check_punct(iter: &mut IntoIter, expect: char) -> Result<Punct, SolveError> {
    let punct = get_punct(iter)?;
    if punct.as_char() != expect {
        Err(SolveError::Syntax(TokenTree::Punct(punct)))
    } else {
        Ok(punct)
    }
}

/// Get the next token and expect it to be a word
fn get_word(iter: &mut IntoIter) -> Result<Ident, SolveError> {
    if let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(word) => Ok(word),
            _ => Err(SolveError::Syntax(token)),
        }
    } else {
        Err(SolveError::End)
    }
}

/// Check if the next token is the specified word
fn check_word(iter: &mut IntoIter, expect: &str) -> Result<Ident, SolveError> {
    let word = get_word(iter)?;
    if word != expect {
        Err(SolveError::Syntax(TokenTree::Ident(word)))
    } else {
        Ok(word)
    }
}
