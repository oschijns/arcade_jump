use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Float};

/// Compute a vertical impulse parameter from the peak height and the time to reach the peak
#[inline]
pub fn time_from_height_and_impulse<N: 'static + NumOps + Copy>(h: N, v: N) -> N
where
    isize: AsPrimitive<N>,
{
    2.as_() * h / v
}

/// Compute a vertical impulse parameter from the peak height and the gravity
#[inline]
pub fn time_from_height_and_gravity<
    N: 'static + NumOps + Copy + AsPrimitive<F>,
    F: Float + AsPrimitive<N>,
>(
    h: N,
    g: N,
) -> N
where
    isize: AsPrimitive<N>,
{
    let f: F = (2.as_() * h / g).as_();
    f.abs().sqrt().as_()
}

/// Compute a vertical impulse parameter from the time to reach the peak and the gravity
#[inline]
pub fn time_from_impulse_and_gravity<N: 'static + NumOps + Copy + Neg<Output = N>>(
    v: N,
    g: N,
) -> N {
    -v / g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_v() {
        assert_eq!(time_from_height_and_impulse(20.0, 10.0), 4.0);
    }

    #[test]
    fn test_from_h_g() {
        assert_eq!(time_from_height_and_gravity::<i32, f32>(20, -1), 6);
    }

    #[test]
    fn test_from_v_g() {
        assert_eq!(time_from_impulse_and_gravity(10.0, -1.0), 10.0);
    }
}
