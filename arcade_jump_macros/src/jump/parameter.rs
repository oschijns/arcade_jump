use super::{ParseTokens, SolveError, check_punct, config::FloatType};
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote;
use std::borrow::Cow;

/// Parameter trait
pub(crate) trait Parameter {
    /// Get the identifier for this parameter
    fn get_ident(&self, index: usize) -> Cow<'_, Ident>;

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType;

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self);
}

/// Input parameter which  and a type
#[derive(Debug, Clone)]
pub(crate) struct ParameterInput {
    /// Either a identifier, a literal or an expression
    variable_input: VariableInput,

    /// Type of the parameter
    parameter_type: ParameterType,
}

/// Either an identifier, a literal or an expression
#[derive(Debug, Clone)]
pub(crate) enum VariableInput {
    /// Directly named variable
    Ident(Ident),

    /// Literal
    Literal(Literal),

    /// Expression
    Expr(Group),
}

/// Output parameter with a name and a type
#[derive(Debug, Clone)]
pub(crate) struct ParameterOutput {
    /// Name of the variable
    variable_name: Ident,

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
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `my_height: Height`
        // `40.0: Height`
        // `(my_height * 2.0): H`

        if let Some(token) = iter.next() {
            let variable_input = VariableInput::try_from(token)?;
            let _ = check_punct(iter, ':')?;
            let parameter_type = ParameterType::parse(iter)?;
            Ok(Self {
                variable_input,
                parameter_type,
            })
        } else {
            Err(SolveError::End)
        }
    }
}

impl ParseTokens for ParameterOutput {
    /// Parse `ident:ident` from the token stream to deduce a parameter
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `my_impulse: Impulse`

        if let Some(token) = iter.next() {
            if let TokenTree::Ident(variable_name) = token {
                let _ = check_punct(iter, ':')?;
                let parameter_type = ParameterType::parse(iter)?;
                Ok(Self {
                    variable_name,
                    parameter_type,
                })
            } else {
                Err(SolveError::Syntax(token))
            }
        } else {
            Err(SolveError::End)
        }
    }
}

impl ParseTokens for ParameterType {
    /// Parse an identifier from the token stream to deduce the parameter type
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        if let Some(token) = iter.next() {
            if let TokenTree::Ident(name) = token {
                Self::try_from(name.to_string().as_str())
            } else {
                Err(SolveError::Syntax(token))
            }
        } else {
            Err(SolveError::End)
        }
    }
}

/// How is the input variable defined ?
impl TryFrom<TokenTree> for VariableInput {
    type Error = SolveError;

    fn try_from(token: TokenTree) -> Result<Self, SolveError> {
        match token {
            TokenTree::Literal(literal) => Ok(Self::Literal(literal)),
            TokenTree::Ident(ident) => Ok(Self::Ident(ident)),
            TokenTree::Group(group) => Ok(Self::Expr(group)),
            _ => Err(SolveError::Syntax(token)),
        }
    }
}

/// Identify the parameter
impl TryFrom<&str> for ParameterType {
    type Error = SolveError;

    #[rustfmt::skip]
    fn try_from(name: &str) -> Result<Self, SolveError> {
        match name {
            "H" | "Height"  => Ok(Self::Height ),
            "T" | "Time"    => Ok(Self::Time   ),
            "I" | "Impulse" => Ok(Self::Impulse),
            "G" | "Gravity" => Ok(Self::Gravity),
            _ => Err(SolveError::Syntax(TokenTree::Ident(Ident::new(
                name,
                Span::call_site(),
            )))),
        }
    }
}

impl ParameterInput {
    /// Preevaluate input expressions once
    pub(crate) fn pre_evaluate(&self, float_type: &FloatType, index: usize) -> TokenStream {
        let let_const = float_type.let_const_token();
        let float = float_type.get_float_type();

        match &self.variable_input {
            VariableInput::Ident(_) => TokenStream::new(),
            VariableInput::Literal(literal) => {
                let param = self.get_ident(index).into_owned();
                quote![ #let_const #param : #float = #literal as #float; ]
            }
            VariableInput::Expr(expr) => {
                let param = self.get_ident(index).into_owned();
                quote![ #let_const #param : #float = #expr as #float; ]
            }
        }
    }
}

impl Parameter for ParameterType {
    /// Get the identifier for this parameter
    #[rustfmt::skip]
    fn get_ident(&self, index: usize) -> Cow<'_, Ident> {
        let name = match self {
            Self::Height  => "height" ,
            Self::Time    => "time"   ,
            Self::Impulse => "impulse",
            Self::Gravity => "gravity",
        };
        Cow::Owned(Ident::new(&format!["__{}{}", name, index], Span::call_site()))
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
    fn get_ident(&self, index: usize) -> Cow<'_, Ident> {
        match &self.variable_input {
            VariableInput::Ident(ident) => Cow::Borrowed(ident),
            _ => self.parameter_type.get_ident(index),
        }
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
    fn get_ident(&self, _: usize) -> Cow<'_, Ident> {
        Cow::Borrowed(&self.variable_name)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let tokens1 = quote![ my_height  : Height  ];
        let tokens2 = quote![ my_time    : Time    ];
        let tokens3 = quote![ my_impulse : Impulse ];
        let tokens4 = quote![ my_gravity : Gravity ];

        let my_height = ParameterInput::parse(&mut tokens1.into_iter()).unwrap();
        let my_time = ParameterInput::parse(&mut tokens2.into_iter()).unwrap();
        let my_impulse = ParameterInput::parse(&mut tokens3.into_iter()).unwrap();
        let my_gravity = ParameterInput::parse(&mut tokens4.into_iter()).unwrap();

        assert_eq!(my_height.get_ident(0).as_ref(), "my_height");
        assert_eq!(my_time.get_ident(0).as_ref(), "my_time");
        assert_eq!(my_impulse.get_ident(0).as_ref(), "my_impulse");
        assert_eq!(my_gravity.get_ident(0).as_ref(), "my_gravity");
    }
}
