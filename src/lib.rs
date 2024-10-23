#![cfg_attr(not(test), no_std)]

/// Compute peak height
pub mod height;

/// Compute time to peak
pub mod time;

/// Compute impulse
pub mod impulse;

/// Compute gravity
pub mod gravity;

/// Compute horizontal range
pub mod horizontal;

/// Compute parameters of a jump trajectory
pub mod solve;

/// Provide a complete implementation of a jump trajectory for video games
#[cfg(feature = "trajectory")]
pub mod jump;

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
