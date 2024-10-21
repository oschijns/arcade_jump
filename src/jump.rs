use crate::gravity::Gravity;
use crate::height::PeakHeight;
use crate::impulse::Impulse;
use crate::time::TimeToPeak;

/// Compute parameters from other parameters
pub trait JumpParameter {
    fn from_height_and_time(h: Self, t: Self) -> (Self, Self)
    where
        Self: Sized;

    fn from_height_and_impulse(h: Self, v: Self) -> (Self, Self)
    where
        Self: Sized;

    fn from_height_and_gravity(h: Self, g: Self) -> (Self, Self)
    where
        Self: Sized;

    fn from_time_and_impulse(t: Self, v: Self) -> (Self, Self)
    where
        Self: Sized;

    fn from_time_and_gravity(t: Self, g: Self) -> (Self, Self)
    where
        Self: Sized;

    fn from_impulse_and_gravity(v: Self, g: Self) -> (Self, Self)
    where
        Self: Sized;
}

macro_rules! impl_jump {
    ($typ:ty : $float:ident) => {
        impl JumpParameter for $typ {
            #[inline]
            fn from_height_and_time(h: Self, t: Self) -> (Self, Self) {
                let v = <Self as Impulse>::from_height_and_time(h, t);
                let g = <Self as Gravity>::from_height_and_time(h, t);
                (v, g)
            }

            #[inline]
            fn from_height_and_impulse(h: Self, v: Self) -> (Self, Self) {
                let t = <Self as TimeToPeak>::from_height_and_impulse(h, v);
                let g = <Self as Gravity>::from_height_and_impulse(h, v);
                (t, g)
            }

            #[inline]
            fn from_height_and_gravity(h: Self, g: Self) -> (Self, Self) {
                let t = <Self as TimeToPeak>::from_height_and_gravity(h, g);
                let v = <Self as Impulse>::from_height_and_gravity(h, g);
                (t, v)
            }

            #[inline]
            fn from_time_and_impulse(t: Self, v: Self) -> (Self, Self) {
                let h = <Self as PeakHeight>::from_time_and_impulse(t, v);
                let g = <Self as Gravity>::from_time_and_impulse(t, v);
                (h, g)
            }

            #[inline]
            fn from_time_and_gravity(t: Self, g: Self) -> (Self, Self) {
                let h = <Self as PeakHeight>::from_time_and_gravity(t, g);
                let v = <Self as Impulse>::from_time_and_gravity(t, g);
                (h, v)
            }

            #[inline]
            fn from_impulse_and_gravity(v: Self, g: Self) -> (Self, Self) {
                let h = <Self as PeakHeight>::from_impulse_and_gravity(v, g);
                let t = <Self as TimeToPeak>::from_impulse_and_gravity(v, g);
                (h, t)
            }
        }
    };
}

impl_jump![f32   : f32];
impl_jump![f64   : f64];
impl_jump![i8    : f32];
impl_jump![i16   : f32];
impl_jump![i32   : f32];
impl_jump![i64   : f64];
impl_jump![i128  : f64];
impl_jump![isize : f64];

#[cfg(test)]
mod tests {
    use super::JumpParameter;

    #[test]
    fn test_from_h_t() {
        let (impulse, gravity) = f32::from_height_and_time(20.0, 10.0);
        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);
    }

    #[test]
    fn test_from_h_v() {
        let (time, gravity) = f32::from_height_and_impulse(20.0, 10.0);
        assert_eq!(time, 4.0);
        assert_eq!(gravity, -2.5);
    }

    #[test]
    fn test_from_h_g() {
        let (time, impulse) = f32::from_height_and_gravity(20.0, 10.0);
        assert_eq!(time, 2.0);
        assert_eq!(impulse, 20.0);
    }

    #[test]
    fn test_from_t_v() {
        let (height, gravity) = f32::from_time_and_impulse(10.0, 20.0);
        assert_eq!(height, 100.0);
        assert_eq!(gravity, -2.0);
    }

    #[test]
    fn test_from_t_g() {
        let (height, impulse) = f32::from_time_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(impulse, 10.0);
    }

    #[test]
    fn test_from_v_g() {
        let (height, time) = f32::from_impulse_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(time, 10.0);
    }
}
