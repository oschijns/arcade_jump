use crate::math::pow2;
use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Zero};

/// Compute the peak height from the time to reach the peak and the vertical impulse
#[inline]
pub fn height_from_time_and_impulse<N: 'static + NumOps + Copy>(t: N, v: N) -> N
where
    isize: AsPrimitive<N>,
{
    v * t / 2.as_()
}

/// Compute the peak height from the time to reach the peak and the gravity
#[inline]
pub fn height_from_time_and_gravity<N: 'static + NumOps + Copy + Neg<Output = N>>(t: N, g: N) -> N
where
    isize: AsPrimitive<N>,
{
    -g * pow2![t] / 2.as_()
}

/// Compute the peak height from the vertical impulse and the gravity
#[inline]
pub fn height_from_impulse_and_gravity<
    N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>,
>(
    v: N,
    g: N,
) -> N
where
    isize: AsPrimitive<N>,
{
    if g.is_zero() {
        N::default()
    } else {
        -pow2![v] / (g * 2.as_())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_t_v() {
        assert_eq!(height_from_time_and_impulse(10.0, 20.0), 100.0);
    }

    #[test]
    fn test_from_t_g() {
        assert_eq!(height_from_time_and_gravity(10.0, -1.0), 50.0);
    }

    #[test]
    fn test_from_v_g() {
        assert_eq!(height_from_impulse_and_gravity(20.0, -1.0), 200.0);
        assert_eq!(height_from_impulse_and_gravity(10, 0), 0);
    }
}
