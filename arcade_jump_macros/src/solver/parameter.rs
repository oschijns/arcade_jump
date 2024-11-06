use super::{get_punct, let_const_token, ParseTokens, SolveError};
use proc_macro2::{token_stream::IntoIter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use std::borrow::Cow;
use syn::Type;

/// Parameter trait
pub(crate) trait Parameter {
    /// Get the identifier for this parameter
    fn get_ident(&self) -> Cow<'_, Ident>;

    /// Get the type of this parameter
    fn get_type(&self) -> ParameterType;

    /// Reorder two parameters
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self);
}

/// Input parameter which  and a type
pub(crate) struct ParameterInput {
    /// Either a identifier, a literal or an expression
    variable_input: VariableInput,

    /// Type of the parameter
    parameter_type: ParameterType,
}

/// Either an identifier, a literal or an expression
pub(crate) enum VariableInput {
    /// Directly named variable
    Ident(Ident),

    /// Literal
    Literal(Literal),

    /// Expression
    Expr(Group),
}

/// Output parameter with a name and a type
pub(crate) struct ParameterOutput {
    /// Name of the variable
    variable_name: Ident,

    /// Type of the parameter
    parameter_type: ParameterType,
}

/// Parameter type
#[repr(u32)]
#[derive(Clone, Copy)]
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
            if get_punct(iter)?.as_char() != ':' {
                return Err(SolveError::Syntax);
            }
            let parameter_type = ParameterType::parse(iter)?;
            Ok(Self {
                variable_input,
                parameter_type,
            })
        } else {
            Err(SolveError::Syntax)
        }
    }
}

impl ParseTokens for ParameterOutput {
    /// Parse `ident:ident` from the token stream to deduce a parameter
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `my_impulse: Impulse`

        if let Some(TokenTree::Ident(variable_name)) = iter.next() {
            if get_punct(iter)?.as_char() != ':' {
                return Err(SolveError::Syntax);
            }
            let parameter_type = ParameterType::parse(iter)?;
            Ok(Self {
                variable_name,
                parameter_type,
            })
        } else {
            Err(SolveError::Syntax)
        }
    }
}

impl ParseTokens for ParameterType {
    /// Parse an identifier from the token stream to deduce the parameter type
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        if let Some(TokenTree::Ident(name)) = iter.next() {
            Self::try_from(name.to_string().as_str())
        } else {
            Err(SolveError::Syntax)
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
            _ => Err(SolveError::Syntax),
        }
    }
}

/// Identify the parameter
impl TryFrom<&str> for ParameterType {
    type Error = SolveError;

    fn try_from(name: &str) -> Result<Self, SolveError> {
        match name {
            "H" | "Height" => Ok(Self::Height),
            "T" | "Time" => Ok(Self::Time),
            "I" | "Impulse" => Ok(Self::Impulse),
            "G" | "Gravity" => Ok(Self::Gravity),
            _ => Err(SolveError::Syntax),
        }
    }
}

impl ParameterInput {
    /// Preevaluate input expressions once
    pub(crate) fn pre_evaluate(&self, is_const: bool, float: &Type) -> TokenStream {
        match &self.variable_input {
            VariableInput::Ident(_) => TokenStream::new(),
            VariableInput::Literal(literal) => {
                let let_const = let_const_token(is_const);
                let param = self.get_ident().into_owned();
                quote![ #let_const #param : #float = #literal as #float; ]
            }
            VariableInput::Expr(expr) => {
                let let_const = let_const_token(is_const);
                let param = self.get_ident().into_owned();
                quote![ #let_const #param : #float = #expr as #float; ]
            }
        }
    }
}

impl Parameter for ParameterType {
    /// Get the identifier for this parameter
    fn get_ident(&self) -> Cow<'_, Ident> {
        match self {
            Self::Height => Cow::Owned(Ident::new("__height", Span::call_site())),
            Self::Time => Cow::Owned(Ident::new("__time", Span::call_site())),
            Self::Impulse => Cow::Owned(Ident::new("__impulse", Span::call_site())),
            Self::Gravity => Cow::Owned(Ident::new("__gravity", Span::call_site())),
        }
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
    fn get_ident(&self) -> Cow<'_, Ident> {
        match &self.variable_input {
            VariableInput::Ident(ident) => Cow::Borrowed(ident),
            _ => self.parameter_type.get_ident(),
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
    fn get_ident(&self) -> Cow<'_, Ident> {
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

        let my_height = ParameterInput::parse(tokens1);
    }

    #[test]
    fn test_parse_output() {
        let param1 = Parameter::new("impulse", Type::Impulse);
        let param2 = Parameter::new("gravity", Type::Gravity);
    }
}
