use crate::math::pow2;

/// Compute gravity coefficient from other parameters
pub trait Gravity {
    fn from_height_and_time(h: Self, t: Self) -> Self;

    fn from_height_and_impulse(h: Self, v: Self) -> Self;

    fn from_time_and_impulse(t: Self, v: Self) -> Self;
}

macro_rules! impl_gravity {
    ($typ:ty) => {
        impl Gravity for $typ {
            #[inline]
            fn from_height_and_time(h: Self, t: Self) -> Self {
                -2 as $typ * h / pow2![t]
            }

            #[inline]
            fn from_height_and_impulse(h: Self, v: Self) -> Self {
                -pow2![v] / (h * 2 as $typ)
            }

            #[inline]
            fn from_time_and_impulse(t: Self, v: Self) -> Self {
                -v / t
            }
        }
    };
}

impl_gravity![f32];
impl_gravity![f64];
impl_gravity![i8];
impl_gravity![i16];
impl_gravity![i32];
impl_gravity![i64];
impl_gravity![i128];
impl_gravity![isize];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        assert_eq!(f32::from_height_and_time(20.0, 10.0), -0.4);
    }

    #[test]
    fn test_from_h_v() {
        assert_eq!(f32::from_height_and_impulse(20.0, 10.0), -2.5);
    }

    #[test]
    fn test_from_t_v() {
        assert_eq!(f32::from_time_and_impulse(10.0, 20.0), -2.0);
    }
}
