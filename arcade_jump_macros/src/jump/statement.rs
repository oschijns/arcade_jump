use super::{
    CompError, ParseTokens,
    parameter::{ParameterInput, ParameterOutput},
    select::select_function,
};
use crate::utils::{check_punct, check_word, to_stream};
use proc_macro2::{Spacing, TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote;

/// A statement taking two parameters and resulting into one or two other parameters
pub(crate) struct Statement {
    /// Numeric type to use
    num_type: Option<TokenStream>,

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
    fn parse(iter: &mut IntoIter) -> Result<Self, CompError> {
        // We expect statements in the form:
        // `Height(my_height), Time(my_time) => Impulse`
        // `H(my_height), T(my_time) => I, G as fixed::FixNum`

        // Read two inputs
        let input1 = ParameterInput::parse(iter)?;
        let _ = check_punct(iter, ',')?;
        let input2 = ParameterInput::parse(iter)?;

        // verify that the two parts are separated by a "=>"
        let arrow = check_punct(iter, '=')?;
        if arrow.spacing() == Spacing::Joint {
            let _ = check_punct(iter, '>')?;
        } else {
            return Err(CompError::MissingArrow);
        }

        // Read a first output
        let output1 = ParameterOutput::parse(iter)?;

        // There may be a second output
        let mut output2 = None;

        // There may be a numerical type
        let mut num_type = None;

        // There is either a comma followed by a second output or an `as` keyword followed by a type
        if let Some(token) = iter.next() {
            match token {
                // there is a second output
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    // get a second parameter
                    output2 = Some(ParameterOutput::parse(iter)?);

                    // check if there is a numerical type too
                    if check_word(iter, "as").is_ok() {
                        num_type = Some(to_stream(iter));
                    }
                }
                // there is an enforced numerical type
                TokenTree::Ident(ident) if ident == "as" => {
                    num_type = Some(to_stream(iter));
                }
                _ => return Err(CompError::InvalidEnd(token)),
            }
        }

        // return a statement
        Ok(Statement {
            num_type,
            input1,
            input2,
            output1,
            output2,
        })
    }
}

impl Statement {
    /// Convert the statement to a token stream
    pub(crate) fn to_tokens(&self) -> Result<TokenStream, CompError> {
        // is there a numerical type enforced ?
        let enforce_type = self.num_type.is_some();
        let num_type = &self.num_type;

        // pre-evaluate the input variables
        let in1 = self.input1.pre_evaluate(enforce_type);
        let in2 = self.input2.pre_evaluate(enforce_type);

        // evaluate the first output result
        let out1 = select_function(enforce_type, &self.input1, &self.input2, &self.output1)?;

        // either we return a single value or a pair of values
        let tokens = if let Some(output2) = &self.output2 {
            let out2 = select_function(enforce_type, &self.input1, &self.input2, output2)?;
            let result = pair_output(&out1, &out2);

            if enforce_type {
                quote![
                    {
                        type __Num = #num_type;
                        #in1
                        #in2
                        #result
                    }
                ]
            } else {
                quote![
                    {
                        #in1
                        #in2
                        #result
                    }
                ]
            }
        } else if enforce_type {
            quote![
                {
                    type __Num = #num_type;
                    #in1
                    #in2
                    #out1
                }
            ]
        } else {
            quote![
                {
                    #in1
                    #in2
                    #out1
                }
            ]
        };

        Ok(tokens)
    }
}

fn pair_output(out1: &TokenStream, out2: &TokenStream) -> TokenStream {
    quote![
        {
            use ::core::result::Result as __Res;
            match (#out1, #out2) {
                (__Res::Ok(__r0), __Res::Ok(__r1)) => __Res::Ok((__r0, __r1)),
                (__Res::Err(__err), _) | (_, __Res::Err(__err)) => __Res::Err(__err),
            }
        }
    ]
}
