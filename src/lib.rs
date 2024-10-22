#![cfg_attr(not(test), no_std)]

/// Compute peak height
pub mod height;

/// Compute time to peak
pub mod time;

/// Compute impulse
pub mod impulse;

/// Compute gravity
pub mod gravity;

/// Compute parameters of a jump trajectory
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

#[macro_export]
macro_rules! solve {
    ({$height:expr, $time:expr, ?, ?} as $typ:ty) => {
        $crate::jump::from_height_and_time(($height) as $typ, ($time) as $typ)
    };
    ({$height:expr, ?, $impulse:expr, ?} as $typ:ty) => {
        $crate::jump::from_height_and_impulse(($height) as $typ, ($impulse) as $typ)
    };
    ({$height:expr, ?, ?, $gravity:expr} as $typ:ty) => {
        $crate::jump::from_height_and_gravity::<$typ, f64>(($height) as $typ, ($gravity) as $typ)
    };
    ({?, $time:expr, $impulse:expr, ?} as $typ:ty) => {
        $crate::jump::from_time_and_impulse(($time) as $typ, ($impulse) as $typ)
    };
    ({?, $time:expr, ?, $gravity:expr} as $typ:ty) => {
        $crate::jump::from_time_and_gravity(($time) as $typ, ($gravity) as $typ)
    };
    ({?, ?, $impulse:expr, $gravity:expr} as $typ:ty) => {
        $crate::jump::from_impulse_and_gravity(($impulse) as $typ, ($gravity) as $typ)
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
