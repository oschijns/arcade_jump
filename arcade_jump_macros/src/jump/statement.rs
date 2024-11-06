use super::{
    check_punct,
    config::FloatType,
    get_punct,
    parameter::{ParameterInput, ParameterOutput},
    select::select_function,
    ParseTokens, SolveError,
};
use proc_macro2::{token_stream::IntoIter, Spacing, TokenStream, TokenTree};
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
        // `my_height: Height, my_time: Time => my_impulse: Impulse;`
        // `my_height: H, my_time: T => my_impulse: I, my_gravity: G;`

        // Read two inputs
        let input1 = ParameterInput::parse(iter)?;
        let _ = check_punct(iter, ',')?;
        let input2 = ParameterInput::parse(iter)?;

        // verify that the two parts are separated by a "=>"
        let arrow = check_punct(iter, '=')?;
        if arrow.spacing() == Spacing::Joint {
            let _ = check_punct(iter, '>')?;
        } else {
            return Err(SolveError::Syntax(TokenTree::Punct(arrow)));
        }

        // Read a first output
        let output1 = ParameterOutput::parse(iter)?;

        // either there is a second output or we stop there
        let punct = get_punct(iter)?;
        let output2 = match punct.as_char() {
            ',' => {
                let output = ParameterOutput::parse(iter)?;
                let _ = check_punct(iter, ';')?;
                Some(output)
            }
            ';' => None,
            _ => return Err(SolveError::Syntax(TokenTree::Punct(punct))),
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
        float_type: &FloatType,
        index: usize,
    ) -> Result<TokenStream, SolveError> {
        // evaluate the first output result
        let out1 = select_function(float_type, index, &self.input1, &self.input2, &self.output1)?;

        // evaluate the second output result
        let out2 = if let Some(output) = &self.output2 {
            select_function(float_type, index, &self.input1, &self.input2, output)?
        } else {
            TokenStream::new()
        };

        // pre-evaluate the input variables (if necessary)
        let in1 = self.input1.pre_evaluate(float_type, index);
        let in2 = self.input2.pre_evaluate(float_type, index);

        Ok(quote![ #in1 #in2 #out1 #out2 ])
    }
}
