use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote;
use syn::{Type, parse_str};

use super::{ParseTokens, SolveError, check_punct, check_word, get_word};

/// Specify the float types (f32 or f64) and the module to use
pub(crate) struct FloatType {
    /// Are values evaluated at compile time or runtime
    is_const: bool,

    /// Primitive float type to use
    float_type: Type,
}

impl FloatType {
    /// Create a new float type
    pub(crate) fn new(is_const: bool, float_type: &str) -> Self {
        Self {
            is_const,
            float_type: parse_str(float_type).unwrap(),
        }
    }

    /// Are values evaluated at compile time?
    #[inline]
    pub(crate) fn is_const(&self) -> bool {
        self.is_const
    }

    /// Return either `let` or `const` token
    #[inline]
    pub(crate) fn let_const_token(&self) -> TokenStream {
        if self.is_const {
            quote![const]
        } else {
            quote![let]
        }
    }

    /// Primitive float type to use
    #[inline]
    pub(crate) fn get_float_type(&self) -> &Type {
        &self.float_type
    }
}

impl ParseTokens for FloatType {
    /// Read a `use f32;`
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect a statement of the form:
        // `use const f64;`
        // `use f32;`
        let _ = check_word(iter, "use")?;

        // next token is either `const` or directly the float type
        let mut word = get_word(iter)?;
        let is_const = if word == "const" {
            // next word is the float type
            word = get_word(iter)?;
            true
        } else {
            // already the float type
            false
        };

        // the statement ends with a `;`
        let _ = check_punct(iter, ';')?;

        // evaluate the float type to use
        match word.to_string().as_str() {
            "f32" => Ok(Self::new(is_const, "f32")),
            "f64" => Ok(Self::new(is_const, "f64")),
            _ => Err(SolveError::Syntax(TokenTree::Ident(word))),
        }
    }
}
