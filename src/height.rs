use crate::math::pow2;

/// Compute peak height coefficient from other parameters
pub trait PeakHeight {
    fn from_time_and_impulse(t: Self, v: Self) -> Self;

    fn from_time_and_gravity(t: Self, g: Self) -> Self;

    fn from_impulse_and_gravity(v: Self, g: Self) -> Self;
}

macro_rules! impl_height {
    ($typ:ty) => {
        impl PeakHeight for $typ {
            #[inline]
            fn from_time_and_impulse(t: Self, v: Self) -> Self {
                v * t / 2 as $typ
            }

            #[inline]
            fn from_time_and_gravity(t: Self, g: Self) -> Self {
                -g * pow2![t] / 2 as $typ
            }

            #[inline]
            fn from_impulse_and_gravity(v: Self, g: Self) -> Self {
                -pow2![v] / (g * 2 as $typ)
            }
        }
    };
}

impl_height![f32];
impl_height![f64];
impl_height![i8];
impl_height![i16];
impl_height![i32];
impl_height![i64];
impl_height![i128];
impl_height![isize];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_t_v() {
        assert_eq!(f32::from_time_and_impulse(10.0, 20.0), 100.0);
    }

    #[test]
    fn test_from_t_g() {
        assert_eq!(f32::from_time_and_gravity(10.0, -1.0), 50.0);
    }

    #[test]
    fn test_from_v_g() {
        assert_eq!(f32::from_impulse_and_gravity(20.0, -1.0), 200.0);
    }
}
