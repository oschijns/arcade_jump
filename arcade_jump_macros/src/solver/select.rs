use super::{
    parameter::{ParameterInput, ParameterOutput},
    FloatType, SolveError,
};
use crate::solver::{
    let_const_token,
    parameter::{Parameter, ParameterType},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Select the function that will give the result for
/// this parameter type given the two other parameter.
pub fn select_function(
    is_const: bool,
    float_type: &FloatType,
    param1: &ParameterInput,
    param2: &ParameterInput,
    output: &ParameterOutput,
) -> Result<TokenStream, SolveError> {
    // find the appropriate function to call
    type Type = ParameterType;
    let func_name = match output.get_type() {
        Type::Height => match param1.get_type() {
            Type::Time => match param2.get_type() {
                Type::Impulse => "height_from_time_and_impulse",
                Type::Gravity => "height_from_time_and_gravity",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Impulse => match param2.get_type() {
                Type::Time => "height_from_time_and_impulse",
                Type::Gravity => "height_from_impulse_and_gravity",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Gravity => match param1.get_type() {
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
        Type::Time => match param1.get_type() {
            Type::Height => match param2.get_type() {
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
            Type::Impulse => match param2.get_type() {
                Type::Height => "time_from_height_and_impulse",
                Type::Gravity => "time_from_impulse_and_gravity",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Gravity => match param2.get_type() {
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
        Type::Impulse => match param1.get_type() {
            Type::Height => match param2.get_type() {
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
            Type::Time => match param2.get_type() {
                Type::Height => "impulse_from_height_and_time",
                Type::Gravity => "impulse_from_time_and_gravity",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Gravity => match param2.get_type() {
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
        Type::Gravity => match param1.get_type() {
            Type::Height => match param2.get_type() {
                Type::Time => "gravity_from_height_and_time",
                Type::Impulse => "gravity_from_height_and_impulse",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Time => match param2.get_type() {
                Type::Height => "gravity_from_height_and_time",
                Type::Impulse => "gravity_from_time_and_impulse",
                _ => {
                    return Err(SolveError::Parameter);
                }
            },
            Type::Impulse => match param2.get_type() {
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
    let eval = let_const_token(is_const);
    let result = output.get_ident().into_owned();
    let float = &float_type.float_type;
    let path = &float_type.module_path;
    let func = Ident::new(func_name, Span::call_site());
    let var1 = ord1.get_ident().into_owned();
    let var2 = ord2.get_ident().into_owned();

    // generate the statement
    Ok(quote![#eval #result: #float = #path::#func(#var1, #var2);])
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
