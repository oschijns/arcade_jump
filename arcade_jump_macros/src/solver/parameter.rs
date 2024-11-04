use super::{check_punct, FloatType, Reorder, SolveError};
use proc_macro2::{token_stream::IntoIter, Ident, Span, TokenStream, TokenTree};
use quote::quote;

/// Parameter with a name and a type
pub(crate) struct Parameter {
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

impl Parameter {
    /// Create a new parameter
    fn new(name: &str, typ: ParameterType) -> Self {
        Self {
            variable_name: Ident::new(name, Span::call_site()),
            parameter_type: typ,
        }
    }
}

impl Parameter {
    /// Parse `ident:ident` from the token stream to deduce a parameter
    pub(crate) fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `height: my_height`
        let parameter_type = ParameterType::parse(iter)?;
        let _ = check_punct(iter, ':')?;
        if let Some(TokenTree::Ident(variable_name)) = iter.next() {
            Ok(Parameter {
                variable_name,
                parameter_type,
            })
        } else {
            Err(SolveError::Syntax)
        }
    }
}

impl ParameterType {
    /// Parse an identifier from the token stream to deduce the parameter type
    pub(crate) fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        if let Some(TokenTree::Ident(name)) = iter.next() {
            Self::try_from(name.to_string().as_str())
        } else {
            Err(SolveError::Syntax)
        }
    }
}

/// Identify the parameter
impl TryFrom<&str> for ParameterType {
    type Error = SolveError;

    fn try_from(value: &str) -> Result<Self, SolveError> {
        match value {
            "height" => Ok(Self::Height),
            "time" => Ok(Self::Time),
            "impulse" => Ok(Self::Impulse),
            "gravity" => Ok(Self::Gravity),
            _ => Err(SolveError::Syntax),
        }
    }
}

/// Reorder parameters
impl Reorder for ParameterType {
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self) {
        if (*self as u32) < (*other as u32) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

/// Reorder parameters
impl Reorder for Parameter {
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self) {
        if (self.parameter_type as u32) < (other.parameter_type as u32) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

impl Parameter {
    /// Select the function that will give the result for
    /// this parameter type given the two other parameter.
    pub fn select_function(
        &self,
        is_const: bool,
        float_type: &FloatType,
        param1: &Self,
        param2: &Self,
    ) -> Result<TokenStream, SolveError> {
        // find the appropriate function to call
        type Type = ParameterType;
        let func_name = match self.parameter_type {
            Type::Height => match param1.parameter_type {
                Type::Time => match param2.parameter_type {
                    Type::Impulse => "height_from_time_and_impulse",
                    Type::Gravity => "height_from_time_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Time => "height_from_time_and_impulse",
                    Type::Gravity => "height_from_impulse_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Gravity => match param1.parameter_type {
                    Type::Time => "height_from_time_and_gravity",
                    Type::Impulse => "height_from_impulse_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Time => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Impulse => "time_from_height_and_impulse",
                    Type::Gravity => {
                        if is_const {
                            "time_from_height_and_gravity_const"
                        } else {
                            "time_from_height_and_gravity"
                        }
                    }
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Height => "time_from_height_and_impulse",
                    Type::Gravity => "time_from_impulse_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Gravity => match param2.parameter_type {
                    Type::Height => {
                        if is_const {
                            "time_from_height_and_gravity_const"
                        } else {
                            "time_from_height_and_gravity"
                        }
                    }
                    Type::Impulse => "time_from_impulse_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Impulse => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Time => "impulse_from_height_and_time",
                    Type::Gravity => {
                        if is_const {
                            "impulse_from_height_and_gravity_const"
                        } else {
                            "impulse_from_height_and_gravity"
                        }
                    }
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Time => match param2.parameter_type {
                    Type::Height => "impulse_from_height_and_time",
                    Type::Gravity => "impulse_from_time_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Gravity => match param2.parameter_type {
                    Type::Height => {
                        if is_const {
                            "impulse_from_height_and_gravity_const"
                        } else {
                            "impulse_from_height_and_gravity"
                        }
                    }
                    Type::Time => "impulse_from_time_and_gravity",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Gravity => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Time => "gravity_from_height_and_time",
                    Type::Impulse => "gravity_from_height_and_impulse",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Time => match param2.parameter_type {
                    Type::Height => "gravity_from_height_and_time",
                    Type::Impulse => "gravity_from_time_and_impulse",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Height => "gravity_from_height_and_impulse",
                    Type::Time => "gravity_from_time_and_impulse",
                    _ => {
                        return Err(SolveError::Parameter);
                    }
                },
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
        };

        // prepare the tokens
        let (ord1, ord2) = param1.reorder(param2);
        let eval = if is_const { quote![const] } else { quote![let] };
        let result = &self.variable_name;
        let float = &float_type.float_type;
        let path = &float_type.module_path;
        let func = Ident::new(func_name, Span::call_site());
        let var1 = &ord1.variable_name;
        let var2 = &ord2.variable_name;

        // generate the statement
        Ok(quote![#eval #result: #float = #path::#func(#var1, #var2);])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_func_select() {
        type Type = ParameterType;

        let float = FloatType::new(
            parse_str("f32").unwrap(),
            parse_str("::arcade_jump::jump_parameter::float32").unwrap(),
        );
        let res = Parameter::new("impulse", Type::Impulse);
        let param1 = Parameter::new("time", Type::Time);
        let param2 = Parameter::new("height", Type::Height);

        let tokens = res
            .select_function(false, &float, &param1, &param2)
            .unwrap();

        assert_eq!(
            tokens.to_string(),
            quote![
                let impulse: f32 = ::arcade_jump::jump_parameter::float32::impulse_from_height_and_time(height, time);
            ]
            .to_string()
        );
    }
}
