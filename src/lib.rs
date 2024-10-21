#![cfg_attr(not(test), no_std)]

/// Compute gravity
mod gravity;

/// Compute peak height
mod height;

/// Compute impulse
mod impulse;

/// Compute parameters of a jump trajectory
mod jump;

/// Compute time to peak
mod time;

pub use gravity::Gravity;
pub use height::PeakHeight;
pub use impulse::Impulse;
pub use jump::JumpParameter;
pub use time::TimeToPeak;

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

    macro_rules! sqrt {
        ($var:ident as f32) => {
            libm::sqrtf(libm::fabsf($var as f32))
        };
        ($var:ident as f64) => {
            libm::sqrt(libm::fabs($var as f64))
        };
        (($val:expr) as f32) => {
            libm::sqrtf(libm::fabsf($val as f32))
        };
        (($val:expr) as f64) => {
            libm::sqrt(libm::fabs($val as f64))
        };
    }

    pub(crate) use pow2;
    pub(crate) use sqrt;
}

#[macro_export]
macro_rules! solve {
    ({$height:expr, $time:expr, ?, ?} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_height_and_time(($height) as $typ, ($time) as $typ)
    };
    ({$height:expr, ?, $impulse:expr, ?} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_height_and_impulse(
            ($height) as $typ,
            ($impulse) as $typ,
        )
    };
    ({$height:expr, ?, ?, $gravity:expr} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_height_and_gravity(
            ($height) as $typ,
            ($gravity) as $typ,
        )
    };
    ({?, $time:expr, $impulse:expr, ?} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_time_and_impulse(($time) as $typ, ($impulse) as $typ)
    };
    ({?, $time:expr, ?, $gravity:expr} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_time_and_gravity(($time) as $typ, ($gravity) as $typ)
    };
    ({?, ?, $impulse:expr, $gravity:expr} as $typ:ty) => {
        <$typ as $crate::JumpParameter>::from_impulse_and_gravity(
            ($impulse) as $typ,
            ($gravity) as $typ,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        let (impulse, gravity) = solve![{20.0, 10.0, ?, ?} as f32];
        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);
    }

    #[test]
    fn test_from_h_v() {
        let (time, gravity) = solve![{20.0, ?, 10.0, ?} as f32];
        assert_eq!(time, 4.0);
        assert_eq!(gravity, -2.5);
    }

    #[test]
    fn test_from_h_g() {
        let (time, impulse) = solve![{20.0, ?, ?, 10.0} as f32];
        assert_eq!(time, 2.0);
        assert_eq!(impulse, 20.0);
    }

    #[test]
    fn test_from_t_v() {
        let (height, gravity) = solve![{?, 10.0, 20.0, ?} as f32];
        assert_eq!(height, 100.0);
        assert_eq!(gravity, -2.0);
    }

    #[test]
    fn test_from_t_g() {
        let (height, impulse) = solve![{?, 10.0, ?, -1.0} as f32];
        assert_eq!(height, 50.0);
        assert_eq!(impulse, 10.0);
    }

    #[test]
    fn test_from_v_g() {
        let (height, time) = solve![{?, ?, 10.0, -1.0} as f32];
        assert_eq!(height, 50.0);
        assert_eq!(time, 10.0);
    }
}
