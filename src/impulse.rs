use crate::math::sqrt;

/// Compute vertical impulse coefficient from other parameters
pub trait Impulse {
    fn from_height_and_time(h: Self, t: Self) -> Self;

    fn from_height_and_gravity(h: Self, g: Self) -> Self;

    fn from_time_and_gravity(t: Self, g: Self) -> Self;
}

macro_rules! impl_impulse {
    ($typ:ty : $float:ident) => {
        impl Impulse for $typ {
            #[inline]
            fn from_height_and_time(h: Self, t: Self) -> Self {
                2 as $typ * h / t
            }

            #[inline]
            fn from_height_and_gravity(h: Self, g: Self) -> Self {
                sqrt![(2 as $typ * h * g) as $float] as Self
            }

            #[inline]
            fn from_time_and_gravity(t: Self, g: Self) -> Self {
                -g * t
            }
        }
    };
}

impl_impulse![f32   : f32];
impl_impulse![f64   : f64];
impl_impulse![i8    : f32];
impl_impulse![i16   : f32];
impl_impulse![i32   : f32];
impl_impulse![i64   : f64];
impl_impulse![i128  : f64];
impl_impulse![isize : f64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        assert_eq!(f32::from_height_and_time(20.0, 10.0), 4.0);
    }

    #[test]
    fn test_from_h_g() {
        assert_eq!(i32::from_height_and_gravity(20, -1), 6);
    }

    #[test]
    fn test_from_t_g() {
        assert_eq!(f32::from_time_and_gravity(10.0, -1.0), 10.0);
    }
}
