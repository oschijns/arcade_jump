use super::{
    get_punct,
    parameter::{ParameterInput, ParameterOutput},
    select::select_function,
    FloatType, ParseTokens, SolveError,
};
use proc_macro2::{token_stream::IntoIter, Spacing, TokenStream};
use quote::quote;

/// A statement taking two parameters and resulting into one or two other parameters
pub(crate) struct Statement {
    /// First input parameter
    input1: ParameterInput,

    /// Second input parameter
    input2: ParameterInput,

    /// First output parameter
    output1: ParameterOutput,

    /// Optional second output parameter
    output2: Option<ParameterOutput>,
}

impl ParseTokens for Statement {
    /// Parse a statement `ident:ident,ident:ident=>ident:ident` from a iterator over tokens
    fn parse(iter: &mut IntoIter) -> Result<Self, SolveError> {
        // We expect statements in the form:
        // `height: my_height, time: my_time => impulse: my_impulse;`
        // `height: my_height, time: my_time => impulse: my_impulse, gravity: my_gravity;`

        // Read two inputs
        let input1 = ParameterInput::parse(iter)?;
        if get_punct(iter)?.as_char() != ',' {
            return Err(SolveError::Syntax);
        }
        let input2 = ParameterInput::parse(iter)?;

        // verify that the two parts are separated by a "=>"
        let arrow = get_punct(iter)?;
        if arrow.as_char() == '=' && arrow.spacing() == Spacing::Joint {
            if get_punct(iter)?.as_char() != '>' {
                return Err(SolveError::Syntax);
            }
        } else {
            return Err(SolveError::Syntax);
        }

        // Read a first output
        let output1 = ParameterOutput::parse(iter)?;

        // either there is a second output or we stop there
        let output2 = match get_punct(iter)?.as_char() {
            ',' => {
                let output = ParameterOutput::parse(iter)?;
                if get_punct(iter)?.as_char() != ';' {
                    return Err(SolveError::Syntax);
                }
                Some(output)
            }
            ';' => None,
            _ => return Err(SolveError::Syntax),
        };

        // return a statement
        Ok(Statement {
            input1,
            input2,
            output1,
            output2,
        })
    }
}

impl Statement {
    /// Convert the statement to a token stream
    pub(crate) fn to_tokens(
        &self,
        is_const: bool,
        float_type: &FloatType,
    ) -> Result<TokenStream, SolveError> {
        // evaluate the first output result
        let out1 = select_function(
            is_const,
            float_type,
            &self.input1,
            &self.input2,
            &self.output1,
        )?;

        // evaluate the second output result
        let out2 = if let Some(output) = &self.output2 {
            select_function(is_const, float_type, &self.input1, &self.input2, output)?
        } else {
            TokenStream::new()
        };

        // pre-evaluate the input variables (if necessary)
        let in1 = self.input1.pre_evaluate(is_const, &float_type.float_type);
        let in2 = self.input2.pre_evaluate(is_const, &float_type.float_type);

        Ok(quote![ #in1 #in2 #out1 #out2 ])
    }
}
