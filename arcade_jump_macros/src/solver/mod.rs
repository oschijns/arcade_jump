/// Input and output parameters
mod parameter;

/// Select the function
mod select;

/// How to read a statement
mod statement;

use proc_macro2::{token_stream::IntoIter, Punct, TokenStream, TokenTree};
use quote::quote;
use statement::Statement;
use syn::{parse_str, Path, Type};

/// Parse token stream and generate instruction to compute variables
pub(crate) fn generate_solver(tokens: TokenStream) -> Result<TokenStream, SolveError> {
    // read the stream of tokens
    let mut iter = tokens.into_iter();

    // Collect the statements
    let mut statements = Vec::<Statement>::new();

    // Read all the statements
    loop {
        match Statement::parse(&mut iter) {
            Ok(stmt) => {
                statements.push(stmt);
            }
            Err(_) => {
                break;
            }
        }
    }

    // Generate the statements
    let float = FloatType::new(
        parse_str("f32").unwrap(),
        parse_str("::arcade_jump::jump_parameter::float32").unwrap(),
    );
    let mut output = TokenStream::new();
    for stmt in statements {
        output.extend(stmt.to_tokens(false, &float)?);
    }

    Ok(output)
}

/// Specify the float types (f32 or f64) and the module to use
pub(crate) struct FloatType {
    /// Primitive float type to use
    float_type: Type,

    /// Path to the module containing the functions
    module_path: Path,
}

impl FloatType {
    /// Create a new float type
    pub(crate) fn new(float_type: Type, module_path: Path) -> Self {
        Self {
            float_type,
            module_path,
        }
    }
}

/// Read a sequence of tokens to get the expected type
pub(crate) trait ParseTokens: Sized {
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError>;
}

/// Check if the next token is the specified punctuation
fn get_punct(iter: &mut IntoIter) -> Result<Punct, SolveError> {
    if let Some(TokenTree::Punct(punct)) = iter.next() {
        Ok(punct)
    } else {
        Err(SolveError::Syntax)
    }
}

/// Return either `let` or `const` token
#[inline]
fn let_const_token(is_const: bool) -> TokenStream {
    if is_const {
        quote![const]
    } else {
        quote![let]
    }
}

/// The type of errors encountered when parsing statements
#[repr(u32)]
#[derive(Debug)]
pub enum SolveError {
    /// Error in the syntax
    Syntax,

    /// Error on a parameter
    Parameter,
}
