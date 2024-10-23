use num::{cast::AsPrimitive, traits::NumOps, Float, Zero};

/// Compute the time to reach the peak from the horizontal speed and the range
#[inline]
pub fn from_speed_and_range<N: 'static + NumOps + Copy + Zero + Default>(s: N, d: N) -> N
where
    isize: AsPrimitive<N>,
{
    if s.is_zero() {
        N::default()
    } else {
        d / (s * 2.as_())
    }
}

/// Compute the time to reach the peak from the horizontal speed, the range and an arbitrary ratio
#[inline]
pub fn from_speed_range_and_ratio<
    N: 'static + NumOps + Copy + Zero + Default + AsPrimitive<F>,
    F: Float + AsPrimitive<N>,
>(
    s: N,
    d: N,
    r: F,
) -> (N, N) {
    if s.is_zero() {
        (N::default(), N::default())
    } else {
        let f: F = d.as_() / s.as_();
        let t1 = f * r;
        let t2 = f * (F::one() - r);
        (t1.as_(), t2.as_())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_s_d() {
        assert_eq!(from_speed_and_range(1.0, 20.0), 10.0);
        assert_eq!(from_speed_and_range(0, 20), 0);
    }

    #[test]
    fn test_from_s_d_r() {
        assert_eq!(from_speed_range_and_ratio(1.0, 25.0, 0.6), (15.0, 10.0));
        assert_eq!(from_speed_range_and_ratio(0, 25, 0.6), (0, 0));
    }
}
