use super::{
    CompError,
    parameter::{Parameter, ParameterInput, ParameterOutput, ParameterType},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Select the function that will give the result for
/// this parameter type given the two other parameter.
pub fn select_function(
    enforce_type: bool,
    param1: &ParameterInput,
    param2: &ParameterInput,
    output: &ParameterOutput,
) -> Result<TokenStream, CompError> {
    // find the appropriate function to call
    type Type = ParameterType;

    // reorder the parameters as: Height, Time, Impulse, Gravity
    let (ord1, ord2) = param1.reorder(param2);

    // figure out if the combination of parameter is valid
    #[rustfmt::skip]
    let func_name = match (ord1.get_type(), ord2.get_type(), output.get_type()) {
        (Type::Height , Type::Time   , Type::Impulse) => Ok("impulse_from_height_and_time"   ),
        (Type::Height , Type::Time   , Type::Gravity) => Ok("gravity_from_height_and_time"   ),
        (Type::Height , Type::Impulse, Type::Time   ) => Ok("time_from_height_and_impulse"   ),
        (Type::Height , Type::Impulse, Type::Gravity) => Ok("gravity_from_height_and_impulse"),
        (Type::Height , Type::Gravity, Type::Time   ) => Ok("time_from_height_and_gravity"   ),
        (Type::Height , Type::Gravity, Type::Impulse) => Ok("impulse_from_height_and_gravity"),
        (Type::Time   , Type::Impulse, Type::Height ) => Ok("height_from_time_and_impulse"   ),
        (Type::Time   , Type::Impulse, Type::Gravity) => Ok("gravity_from_time_and_impulse"  ),
        (Type::Time   , Type::Gravity, Type::Height ) => Ok("height_from_time_and_gravity"   ),
        (Type::Time   , Type::Gravity, Type::Impulse) => Ok("impulse_from_time_and_gravity"  ),
        (Type::Impulse, Type::Gravity, Type::Height ) => Ok("height_from_impulse_and_gravity"),
        (Type::Impulse, Type::Gravity, Type::Time   ) => Ok("time_from_impulse_and_gravity"  ),
        _ => Err(CompError::InvalidCombination {
            input1: param1.clone(),
            input2: param2.clone(),
            output: output.clone(),
        }),
    }?;

    // prepare the tokens
    let func = Ident::new(func_name, Span::call_site());
    let var1 = ord1.get_ident();
    let var2 = ord2.get_ident();

    // generate the statement
    let stmt = if enforce_type {
        quote![
            ::arcade_jump::resolver::#func::<__Num>(#var1, #var2)
        ]
    } else {
        quote![
            ::arcade_jump::resolver::#func(#var1, #var2)
        ]
    };
    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jump::parameter::*;
    use crate::jump::*;
    use alloc::string::ToString;

    #[test]
    fn test_func_select() {
        let tokens1 = quote![Height(my_height)];
        let tokens2 = quote![Time(0.5)];
        let tokens3 = quote![I(3 + 5)];

        let my_height = ParameterInput::parse(&mut tokens1.into_iter()).unwrap();
        let my_time = ParameterInput::parse(&mut tokens2.into_iter()).unwrap();
        let my_impulse = ParameterOutput::parse(&mut tokens3.into_iter()).unwrap();

        let tokens = select_function(true, &my_height, &my_time, &my_impulse).unwrap();

        assert_eq!(
            tokens.to_string(),
            quote![
                ::arcade_jump::resolver::impulse_from_height_and_time::<__Num>(__height, __time)
            ]
            .to_string()
        );
    }
}
