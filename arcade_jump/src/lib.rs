#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_fn_floating_point_arithmetic)]

/// Compute the trajectory of a jump
pub mod jump_parameter;

mod math {

    /// Compute the square of a value
    macro_rules! pow2 {
        ($var:ident) => {
            $var * $var
        };
        ($val:expr) => {{
            let val = $val;
            val * val
        }};
    }
    pub(crate) use pow2;
}
