use super::{
    config::FloatType,
    parameter::{ParameterInput, ParameterOutput},
    SolveError,
};
use crate::solver::parameter::{Parameter, ParameterType};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Select the function that will give the result for
/// this parameter type given the two other parameter.
pub fn select_function(
    float_type: &FloatType,
    index: usize,
    param1: &ParameterInput,
    param2: &ParameterInput,
    output: &ParameterOutput,
) -> Result<TokenStream, SolveError> {
    // find the appropriate function to call
    type Type = ParameterType;

    // reorder the parameters as: Height, Time, Impulse, Gravity
    let (ord1, ord2) = param1.reorder(param2);

    // figure out if the combination of parameter is valid
    #[rustfmt::skip]
    let func_name = match (ord1.get_type(), ord2.get_type(), output.get_type()) {
        (Type::Height, Type::Time   , Type::Impulse) => Ok("impulse_from_height_and_time"    ),
        (Type::Height, Type::Time   , Type::Gravity) => Ok("gravity_from_height_and_time"    ),
        (Type::Height, Type::Impulse, Type::Time   ) => Ok("time_from_height_and_impulse"    ),
        (Type::Height, Type::Impulse, Type::Gravity) => Ok("gravity_from_height_and_impulse" ),
        (Type::Height, Type::Gravity, Type::Time   ) => Ok(if float_type.is_const() {
            "time_from_height_and_gravity_const"
        } else {
            "time_from_height_and_gravity"
        }),
        (Type::Height, Type::Gravity, Type::Impulse) => Ok(if float_type.is_const() {
            "impulse_from_height_and_gravity_const"
        } else {
            "impulse_from_height_and_gravity"
        }),
        (Type::Time   , Type::Impulse, Type::Height ) => Ok("height_from_time_and_impulse"   ),
        (Type::Time   , Type::Impulse, Type::Gravity) => Ok("gravity_from_time_and_impulse"  ),
        (Type::Time   , Type::Gravity, Type::Height ) => Ok("height_from_time_and_gravity"   ),
        (Type::Time   , Type::Gravity, Type::Impulse) => Ok("impulse_from_time_and_gravity"  ),
        (Type::Impulse, Type::Gravity, Type::Height ) => Ok("height_from_impulse_and_gravity"),
        (Type::Impulse, Type::Gravity, Type::Time   ) => Ok("time_from_impulse_and_gravity"  ),
        _ => Err(SolveError::Parameter {
            input1: param1.clone(),
            input2: param2.clone(),
            output: output.clone(),
        }),
    }?;

    // prepare the tokens
    let eval = float_type.let_const_token();
    let result = output.get_ident(index).into_owned();
    let float = float_type.get_float_type();
    let path = float_type.get_module_path();
    let func = Ident::new(func_name, Span::call_site());
    let var1 = ord1.get_ident(index).into_owned();
    let var2 = ord2.get_ident(index).into_owned();

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

        let float = FloatType::new(false, "f32", "::arcade_jump::jump_parameter::float32");
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
