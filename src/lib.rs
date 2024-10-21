#![cfg_attr(not(test), no_std)]

/// Compute gravity
mod gravity;

/// Compute peak height
mod height;

/// Compute impulse
mod impulse;

/// Compute time to peak
mod time;

pub use gravity::Gravity;
pub use height::PeakHeight;
pub use impulse::Impulse;
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
    ({$height:expr, $time:expr, ?, ?} as $typ:ty) => {{
        let height = ($height) as $typ;
        let time = ($time) as $typ;
        let impulse = <$typ as $crate::impulse::Impulse>::from_height_and_time(height, time);
        let gravity = <$typ as $crate::gravity::Gravity>::from_height_and_time(height, time);
        (impulse, gravity)
    }};
    ({$height:expr, ?, $impulse:expr, ?} as $typ:ty) => {{
        let height = ($height) as $typ;
        let impulse = ($impulse) as $typ;
        let time = <$typ as $crate::time::TimeToPeak>::from_height_and_impulse(height, impulse);
        let gravity = <$typ as $crate::gravity::Gravity>::from_height_and_impulse(height, impulse);
        (time, gravity)
    }};
    ({$height:expr, ?, ?, $gravity:expr} as $typ:ty) => {{
        let height = ($height) as $typ;
        let gravity = ($gravity) as $typ;
        let time = <$typ as $crate::time::TimeToPeak>::from_height_and_gravity(height, gravity);
        let impulse = <$typ as $crate::impulse::Impulse>::from_height_and_gravity(height, gravity);
        (time, impulse)
    }};
    ({?, $time:expr, $impulse:expr, ?} as $typ:ty) => {{
        let time = ($time) as $typ;
        let impulse = ($impulse) as $typ;
        let height = <$typ as $crate::height::PeakHeight>::from_time_and_impulse(time, impulse);
        let gravity = <$typ as $crate::gravity::Gravity>::from_time_and_impulse(time, impulse);
        (height, gravity)
    }};
    ({?, $time:expr, ?, $gravity:expr} as $typ:ty) => {{
        let time = ($time) as $typ;
        let gravity = ($gravity) as $typ;
        let height = <$typ as $crate::height::PeakHeight>::from_time_and_gravity(time, gravity);
        let impulse = <$typ as $crate::impulse::Impulse>::from_time_and_gravity(time, gravity);
        (height, impulse)
    }};
    ({?, ?, $impulse:expr, $gravity:expr} as $typ:ty) => {{
        let impulse = ($impulse) as $typ;
        let gravity = ($gravity) as $typ;
        let height =
            <$typ as $crate::height::PeakHeight>::from_impulse_and_gravity(impulse, gravity);
        let time = <$typ as $crate::time::TimeToPeak>::from_impulse_and_gravity(impulse, gravity);
        (height, time)
    }};
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
