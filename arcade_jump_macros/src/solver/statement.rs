use super::{check_punct, parameter::Parameter, FloatType, SolveError};
use proc_macro2::{token_stream::IntoIter, Spacing, TokenStream};
use quote::quote;

/// A statement taking two parameters and resulting into one or two other parameters
pub(crate) struct Statement {
    /// First input parameter
    input1: Parameter,

    /// Second input parameter
    input2: Parameter,

    /// First output parameter
    output1: Parameter,

    /// Optional second output parameter
    output2: Option<Parameter>,
}

impl Statement {
    /// Parse a statement `ident:ident,ident:ident=>ident:ident` from a iterator over tokens
    pub(crate) fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `height: my_height, time: my_time => impulse: my_impulse;`
        // `height: my_height, time: my_time => impulse: my_impulse, gravity: my_gravity;`

        // Read two inputs
        let input1 = Parameter::parse(iter)?;
        let _ = check_punct(iter, ',')?;
        let input2 = Parameter::parse(iter)?;

        // verify that the two parts are separated by a "=>"
        let arrow = check_punct(iter, '=')?;
        if arrow.spacing() == Spacing::Joint {
            let _ = check_punct(iter, '>')?;
        } else {
            return Err(SolveError::Syntax);
        }

        // Read a first output
        let output1 = Parameter::parse(iter)?;

        // Read an optional second output
        let output2 = match Parameter::parse(iter) {
            Ok(output) => Some(output),
            Err(_) => None,
        };

        // check that the statement is terminated by a ';'
        let _ = check_punct(iter, ';');

        // return a statement
        Ok(Statement {
            input1,
            input2,
            output1,
            output2,
        })
    }

    /// Convert the statement to a token stream
    pub(crate) fn to_tokens(
        &self,
        is_const: bool,
        float_type: &FloatType,
    ) -> Result<TokenStream, SolveError> {
        // evaluate the first output result
        let out1 =
            self.output1
                .select_function(is_const, float_type, &self.input1, &self.input2)?;

        // evaluate the second output result
        let out2 = if let Some(output) = &self.output2 {
            output.select_function(is_const, float_type, &self.input1, &self.input2)?
        } else {
            TokenStream::new()
        };

        Ok(quote![ #out1 #out2 ])
    }
}
