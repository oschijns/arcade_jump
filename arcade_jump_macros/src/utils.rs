use proc_macro2::{Ident, Punct, TokenStream, TokenTree, token_stream::IntoIter};
use thiserror::Error;

/// The type of errors encountered when parsing statements
#[derive(Error, Debug)]
pub(crate) enum ParseError {
    /// End of the stream of tokens
    #[error("Unexpected end of the stream")]
    End,

    /// Error in the syntax
    #[error("Unexpected token {0}")]
    Syntax(TokenTree),
}

impl From<ParseError> for syn::Error {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::End => {
                syn::Error::new_spanned(TokenStream::new(), "Unexpected end of the stream")
            }
            ParseError::Syntax(token) => syn::Error::new_spanned(token, "Unexpected token"),
        }
    }
}

/// Get the next token and expect it to be a punctuation
pub(crate) fn get_punct(iter: &mut IntoIter) -> Result<Punct, ParseError> {
    if let Some(token) = iter.next() {
        match token {
            TokenTree::Punct(punct) => Ok(punct),
            _ => Err(ParseError::Syntax(token)),
        }
    } else {
        Err(ParseError::End)
    }
}

/// Check if the next token is the specified punctuation
pub(crate) fn check_punct(iter: &mut IntoIter, expect: char) -> Result<Punct, ParseError> {
    let punct = get_punct(iter)?;
    if punct.as_char() != expect {
        Err(ParseError::Syntax(TokenTree::Punct(punct)))
    } else {
        Ok(punct)
    }
}

/// Get the next token and expect it to be a word
pub(crate) fn get_word(iter: &mut IntoIter) -> Result<Ident, ParseError> {
    if let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(word) => Ok(word),
            _ => Err(ParseError::Syntax(token)),
        }
    } else {
        Err(ParseError::End)
    }
}

/// Check if the next token is the specified word
pub(crate) fn check_word(iter: &mut IntoIter, expect: &str) -> Result<Ident, ParseError> {
    let word = get_word(iter)?;
    if word != expect {
        Err(ParseError::Syntax(TokenTree::Ident(word)))
    } else {
        Ok(word)
    }
}

/// Get the sequence of tokens as a path to a type
#[inline]
pub(crate) fn to_stream(iter: &mut IntoIter) -> TokenStream {
    iter.collect::<TokenStream>()
}
