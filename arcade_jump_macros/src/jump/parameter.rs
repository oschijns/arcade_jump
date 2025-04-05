use super::{CompError, ParseTokens};
use alloc::string::ToString;
use core::fmt;
use proc_macro2::{Ident, Span, TokenStream, TokenTree, token_stream::IntoIter};
use quote::{ToTokens, quote};

/// Parameter trait
pub(crate) trait Parameter {
    /// Get the identifier for this parameter
    fn get_ident(&self) -> Ident;

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType;

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self);
}

/// Input parameter
#[derive(Debug, Clone)]
pub(crate) struct ParameterInput {
    /// Type of the parameter
    parameter_type: ParameterType,

    /// The expression provided as the parameter
    expression_input: TokenStream,
}

/// Output parameter
#[derive(Debug, Clone)]
pub(crate) struct ParameterOutput {
    /// Type of the parameter
    parameter_type: ParameterType,
}

/// Parameter type
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum ParameterType {
    /// Peak height
    Height = 0,

    /// Time to reach the peak
    Time = 1,

    /// Initial vertical impulse
    Impulse = 2,

    /// Gravity force
    Gravity = 3,
}

impl ParseTokens for ParameterInput {
    /// Parse `ident:ident` from the token stream to deduce a parameter
    fn parse(iter: &mut IntoIter) -> Result<Self, CompError> {
        // We expect statements in the form:
        // `Height(my_height)`
        // `H(40.0)`

        let parameter_type = ParameterType::parse(iter)?;
        let expression_input = if let Some(token) = iter.next() {
            match token {
                TokenTree::Group(group) => group.stream(),
                _ => return Err(CompError::InvalidExpr(token)),
            }
        } else {
            return Err(CompError::Missing);
        };
        Ok(Self {
            parameter_type,
            expression_input,
        })
    }
}

impl ParseTokens for ParameterOutput {
    /// Parse an identifier from the token stream to deduce the parameter type
    fn parse(iter: &mut IntoIter) -> Result<Self, CompError> {
        let parameter_type = ParameterType::parse(iter)?;
        Ok(Self { parameter_type })
    }
}

impl ParseTokens for ParameterType {
    /// Parse an identifier from the token stream to deduce the parameter type
    fn parse(iter: &mut IntoIter) -> Result<Self, CompError> {
        if let Some(token) = iter.next() {
            if let TokenTree::Ident(name) = token {
                Self::try_from(name.to_string().as_str())
            } else {
                Err(CompError::InvalidType(token))
            }
        } else {
            Err(CompError::Missing)
        }
    }
}

impl ToTokens for ParameterType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Height => tokens.extend(quote![Height]),
            Self::Time => tokens.extend(quote![Time]),
            Self::Impulse => tokens.extend(quote![Impulse]),
            Self::Gravity => tokens.extend(quote![Gravity]),
        }
    }
}

/// Identify the parameter
impl TryFrom<&str> for ParameterType {
    type Error = CompError;

    #[rustfmt::skip]
    fn try_from(name: &str) -> Result<Self, CompError> {
        match name {
            "H" | "Height"  => Ok(Self::Height ),
            "T" | "Time"    => Ok(Self::Time   ),
            "I" | "Impulse" => Ok(Self::Impulse),
            "G" | "Gravity" => Ok(Self::Gravity),
            _ => Err(CompError::InvalidType(TokenTree::Ident(Ident::new(
                name,
                Span::call_site(),
            )))),
        }
    }
}

impl ParameterInput {
    /// Preevaluate input expressions once
    pub(crate) fn pre_evaluate(&self, enforce_type: bool) -> TokenStream {
        let param = self.get_ident();
        let expr = &self.expression_input;

        // either the type is enforced or it is not
        if enforce_type {
            quote![ let #param = (#expr) as __Num; ]
        } else {
            quote![ let #param = #expr; ]
        }
    }
}

impl Parameter for ParameterType {
    /// Get the identifier for this parameter
    #[rustfmt::skip]
    fn get_ident(&self) -> Ident {
        static HEIGHT  : &str = "__height" ;
        static TIME    : &str = "__time"   ;
        static IMPULSE : &str = "__impulse";
        static GRAVITY : &str = "__gravity";

        let name = match self {
            Self::Height  => HEIGHT,
            Self::Time    => TIME,
            Self::Impulse => IMPULSE,
            Self::Gravity => GRAVITY,
        };
        Ident::new(name, Span::call_site())
    }

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType {
        *self
    }

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self) {
        if (*self as u32) < (*other as u32) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

impl Parameter for ParameterInput {
    /// Get the identifier for this parameter
    fn get_ident(&self) -> Ident {
        self.parameter_type.get_ident()
    }

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType {
        self.parameter_type
    }

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self) {
        if (self.parameter_type as u32) < (other.parameter_type as u32) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

impl Parameter for ParameterOutput {
    /// Get the identifier for this parameter
    fn get_ident(&self) -> Ident {
        Ident::new("__result", Span::call_site())
    }

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType {
        self.parameter_type
    }

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self) {
        if (self.parameter_type as u32) < (other.parameter_type as u32) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

impl fmt::Display for ParameterInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

impl fmt::Display for ParameterOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Height => write!(f, "Height"),
            Self::Time => write!(f, "Time"),
            Self::Impulse => write!(f, "Impulse"),
            Self::Gravity => write!(f, "Gravity"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let tokens1 = quote![Height(my_height)];
        let tokens2 = quote![Time(my_time)];
        let tokens3 = quote![Impulse(my_impulse)];
        let tokens4 = quote![Gravity(my_gravity)];

        let my_height = ParameterInput::parse(&mut tokens1.into_iter()).unwrap();
        let my_time = ParameterInput::parse(&mut tokens2.into_iter()).unwrap();
        let my_impulse = ParameterInput::parse(&mut tokens3.into_iter()).unwrap();
        let my_gravity = ParameterInput::parse(&mut tokens4.into_iter()).unwrap();

        assert_eq!(my_height.get_ident(), "__height");
        assert_eq!(my_time.get_ident(), "__time");
        assert_eq!(my_impulse.get_ident(), "__impulse");
        assert_eq!(my_gravity.get_ident(), "__gravity");
    }
}
