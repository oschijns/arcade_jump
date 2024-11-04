use super::{Reorder, VariableType};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Error;

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

/// Identify the parameter
impl TryFrom<&str> for ParameterType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "height" => Ok(Self::Height),
            "time" => Ok(Self::Time),
            "impulse" => Ok(Self::Impulse),
            "gravity" => Ok(Self::Gravity),
            _ => Err(()),
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
        var_type: &VariableType,
        param1: &Self,
        param2: &Self,
    ) -> Result<TokenStream, Error> {
        // find the appropriate function to call
        type Type = ParameterType;
        let func_name = match self.parameter_type {
            Type::Height => match param1.parameter_type {
                Type::Time => match param2.parameter_type {
                    Type::Impulse => "height_from_time_and_impulse",
                    Type::Gravity => "height_from_time_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute height from given parameters",
                    ))?,
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Time => "height_from_time_and_impulse",
                    Type::Gravity => "height_from_impulse_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute height from given parameters",
                    ))?,
                },
                Type::Gravity => match param1.parameter_type {
                    Type::Time => "height_from_time_and_gravity",
                    Type::Impulse => "height_from_impulse_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute height from given parameters",
                    ))?,
                },
                _ => Err(Error::new_spanned(
                    param1.variable_name.clone(),
                    "Cannot compute height from height",
                ))?,
            },
            Type::Time => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Impulse => "time_from_height_and_impulse",
                    Type::Gravity => {
                        if var_type.is_const() {
                            "time_from_height_and_gravity_const"
                        } else {
                            "time_from_height_and_gravity"
                        }
                    }
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute time from given parameters",
                    ))?,
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Height => "time_from_height_and_impulse",
                    Type::Gravity => "time_from_impulse_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute time from given parameters",
                    ))?,
                },
                Type::Gravity => match param2.parameter_type {
                    Type::Height => {
                        if var_type.is_const() {
                            "time_from_height_and_gravity_const"
                        } else {
                            "time_from_height_and_gravity"
                        }
                    }
                    Type::Impulse => "time_from_impulse_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute time from given parameters",
                    ))?,
                },
                _ => Err(Error::new_spanned(
                    param1.variable_name.clone(),
                    "Cannot compute time from time",
                ))?,
            },
            Type::Impulse => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Time => "impulse_from_height_and_time",
                    Type::Gravity => {
                        if var_type.is_const() {
                            "impulse_from_height_and_gravity_const"
                        } else {
                            "impulse_from_height_and_gravity"
                        }
                    }
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute impulse from given parameters",
                    ))?,
                },
                Type::Time => match param2.parameter_type {
                    Type::Height => "impulse_from_height_and_time",
                    Type::Gravity => "impulse_from_time_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute impulse from given parameters",
                    ))?,
                },
                Type::Gravity => match param2.parameter_type {
                    Type::Height => {
                        if var_type.is_const() {
                            "impulse_from_height_and_gravity_const"
                        } else {
                            "impulse_from_height_and_gravity"
                        }
                    }
                    Type::Time => "impulse_from_time_and_gravity",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute impulse from given parameters",
                    ))?,
                },
                _ => Err(Error::new_spanned(
                    param1.variable_name.clone(),
                    "Cannot compute impulse from impulse",
                ))?,
            },
            Type::Gravity => match param1.parameter_type {
                Type::Height => match param2.parameter_type {
                    Type::Time => "gravity_from_height_and_time",
                    Type::Impulse => "gravity_from_height_and_impulse",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute gravity from given parameters",
                    ))?,
                },
                Type::Time => match param2.parameter_type {
                    Type::Height => "gravity_from_height_and_time",
                    Type::Impulse => "gravity_from_time_and_impulse",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute gravity from given parameters",
                    ))?,
                },
                Type::Impulse => match param2.parameter_type {
                    Type::Height => "gravity_from_height_and_impulse",
                    Type::Time => "gravity_from_time_and_impulse",
                    _ => Err(Error::new_spanned(
                        param2.variable_name.clone(),
                        "Cannot compute gravity from given parameters",
                    ))?,
                },
                _ => Err(Error::new_spanned(
                    param1.variable_name.clone(),
                    "Cannot compute gravity from gravity",
                ))?,
            },
        };

        // prepare the tokens
        let (ord1, ord2) = param1.reorder(param2);
        let res = self.variable_name.clone();
        let func = Ident::new(func_name, Span::call_site());
        let var1 = ord1.variable_name.clone();
        let var2 = ord2.variable_name.clone();

        // generate the statement
        Ok(if let VariableType::Constant(typ) = var_type {
            quote![const #res: #typ = #func(#var1, #var2);]
        } else {
            quote![let #res = #func(#var1, #var2);]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_select() {
        type Type = ParameterType;

        let res = Parameter::new("impulse", Type::Impulse);
        let param1 = Parameter::new("time", Type::Time);
        let param2 = Parameter::new("height", Type::Height);

        let tokens = res
            .select_function(&VariableType::Runtime, &param1, &param2)
            .unwrap();

        assert_eq!(
            tokens.to_string(),
            quote![
                let impulse = impulse_from_height_and_time(height, time);
            ]
            .to_string()
        );
    }
}
