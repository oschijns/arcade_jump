mod parameter;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Type};

/// Parse token stream and generate instruction to compute variables
pub(crate) fn generate_solver(tokens: &TokenStream) -> Result<TokenStream, Error> {
    todo!()
}

/// Specify if the variable is evaluated at runtime or at compile time
enum VariableType {
    /// Runtime evaluation of the variable
    Runtime,

    /// Compile time evaluation of the variable
    Constant(Type),
}

impl VariableType {
    /// The variable type is a compile time constant
    #[inline]
    fn is_const(&self) -> bool {
        match self {
            Self::Runtime => false,
            Self::Constant(_) => true,
        }
    }

    /// The variable is evaluated at runtime
    #[inline]
    fn is_runtime(&self) -> bool {
        match self {
            Self::Runtime => true,
            Self::Constant(_) => false,
        }
    }
}

/// Reorder the two elements
trait Reorder {
    fn reorder<'r>(&'r self, other: &'r Self) -> (&'r Self, &'r Self);
}
