use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Float, Zero};

/// Compute the vertical impulse from the peak height and the time to reach the peak
#[inline]
pub fn impulse_from_height_and_time<N: 'static + NumOps + Copy + Zero + Default>(h: N, t: N) -> N
where
    isize: AsPrimitive<N>,
{
    if t.is_zero() {
        N::default()
    } else {
        2.as_() * h / t
    }
}

/// Compute the vertical impulse from the peak height and the gravity
#[inline]
pub fn impulse_from_height_and_gravity<
    N: 'static + NumOps + Copy + AsPrimitive<F>,
    F: Float + AsPrimitive<N>,
>(
    h: N,
    g: N,
) -> N
where
    isize: AsPrimitive<N>,
{
    let f: F = (2.as_() * h * g).as_();
    f.abs().sqrt().as_()
}

/// Compute the vertical impulse from the time to reach the peak and the gravity
#[inline]
pub fn impulse_from_time_and_gravity<N: 'static + NumOps + Copy + Neg<Output = N>>(
    t: N,
    g: N,
) -> N {
    -g * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        assert_eq!(impulse_from_height_and_time(20.0, 10.0), 4.0);
        assert_eq!(impulse_from_height_and_time(20, 0), 0);
    }

    #[test]
    fn test_from_h_g() {
        assert_eq!(impulse_from_height_and_gravity::<i32, f32>(20, -1), 6);
    }

    #[test]
    fn test_from_t_g() {
        assert_eq!(impulse_from_time_and_gravity(10.0, -1.0), 10.0);
    }
}
