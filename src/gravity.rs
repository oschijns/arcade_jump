use crate::math::pow2;
use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Zero};

/// Compute the gravity from the peak height and the time to reach the peak
#[inline]
pub fn gravity_from_height_and_time<N: 'static + NumOps + Copy + Zero + Default>(h: N, t: N) -> N
where
    isize: AsPrimitive<N>,
{
    if t.is_zero() {
        N::default()
    } else {
        (-2).as_() * h / pow2![t]
    }
}

/// Compute the gravity from the peak height and the vertical impulse
#[inline]
pub fn gravity_from_height_and_impulse<
    N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>,
>(
    h: N,
    v: N,
) -> N
where
    isize: AsPrimitive<N>,
{
    if h.is_zero() {
        N::default()
    } else {
        -pow2![v] / (h * 2.as_())
    }
}

/// Compute the gravity from the time to reach the peak and the vertical impulse
#[inline]
pub fn gravity_from_time_and_impulse<
    N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>,
>(
    t: N,
    v: N,
) -> N {
    if t.is_zero() {
        N::default()
    } else {
        -v / t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        assert_eq!(gravity_from_height_and_time(20.0, 10.0), -0.4);
        assert_eq!(gravity_from_height_and_time(20, 0), 0);
    }

    #[test]
    fn test_from_h_v() {
        assert_eq!(gravity_from_height_and_impulse(20.0, 10.0), -2.5);
        assert_eq!(gravity_from_height_and_impulse(0, 10), 0);
    }

    #[test]
    fn test_from_t_v() {
        assert_eq!(gravity_from_time_and_impulse(10.0, 20.0), -2.0);
        assert_eq!(gravity_from_time_and_impulse(0, 20), 0);
    }
}
