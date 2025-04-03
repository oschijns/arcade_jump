use super::{
    SolveError,
    config::FloatType,
    parameter::{Parameter, ParameterInput, ParameterOutput, ParameterType},
};
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
        _ => Err(SolveError::Parameter {
            input1: param1.clone(),
            input2: param2.clone(),
            output: output.clone(),
        }),
    }?;

    // prepare the tokens
    let eval = float_type.let_const_token();
    let result = output.get_ident(index).into_owned();
    let num_type = float_type.get_float_type();
    let func = Ident::new(func_name, Span::call_site());
    let var1 = ord1.get_ident(index).into_owned();
    let var2 = ord2.get_ident(index).into_owned();

    // generate the statement
    Ok(quote![
        #eval #result = ::arcade_jump::resolver::#func::<#num_type>(#var1, #var2)?;])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jump::parameter::*;
    use crate::jump::*;

    #[test]
    fn test_func_select() {
        let float = FloatType::new(false, "f32");
        let tokens1 = quote![ my_height  : Height  ];
        let tokens2 = quote![ my_time    : Time    ];
        let tokens3 = quote![ my_impulse : Impulse ];

        let my_height = ParameterInput::parse(&mut tokens1.into_iter()).unwrap();
        let my_time = ParameterInput::parse(&mut tokens2.into_iter()).unwrap();
        let my_impulse = ParameterOutput::parse(&mut tokens3.into_iter()).unwrap();

        let tokens = select_function(&float, 0, &my_height, &my_time, &my_impulse).unwrap();

        assert_eq!(
            tokens.to_string(),
            quote![
                let my_impulse = ::arcade_jump::resolver::impulse_from_height_and_time::<f32>(my_height, my_time)?;
            ]
            .to_string()
        );
    }
}
