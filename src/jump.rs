use crate::{gravity::*, height::*, impulse::*, time::*};
use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Float};

/// Compute the vertical impulse and the gravity from the peak height and the time to reach the peak
#[inline]
pub fn from_height_and_time<N: 'static + NumOps + Copy>(h: N, t: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let v = impulse_from_height_and_time(h, t);
    let g = gravity_from_height_and_time(h, t);
    (v, g)
}

/// Compute the time to reach the peak and the gravity from the peak height and the vertical impulse
#[inline]
pub fn from_height_and_impulse<N: 'static + NumOps + Copy + Neg<Output = N>>(h: N, v: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let t = time_from_height_and_impulse(h, v);
    let g = gravity_from_height_and_impulse(h, v);
    (t, g)
}

/// Compute the time to reach the peak and the vertical impulse from the peak height and the gravity
#[inline]
pub fn from_height_and_gravity<
    N: 'static + NumOps + Copy + AsPrimitive<F>,
    F: Float + AsPrimitive<N>,
>(
    h: N,
    g: N,
) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let t = time_from_height_and_gravity(h, g);
    let v = impulse_from_height_and_gravity(h, g);
    (t, v)
}

/// Compute the peak height and the gravity from the time to reach the peak and the vertical impulse
#[inline]
pub fn from_time_and_impulse<N: 'static + NumOps + Copy + Neg<Output = N>>(t: N, v: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let h = height_from_time_and_impulse(t, v);
    let g = gravity_from_time_and_impulse(t, v);
    (h, g)
}

/// Compute the peak height and the vertical impulse from the time to reach the peak and the gravity
#[inline]
pub fn from_time_and_gravity<N: 'static + NumOps + Copy + Neg<Output = N>>(t: N, g: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let h = height_from_time_and_gravity(t, g);
    let v = impulse_from_time_and_gravity(t, g);
    (h, v)
}

/// Compute the peak height and the time to reach the peak from the vertical impulse and the gravity
#[inline]
pub fn from_impulse_and_gravity<N: 'static + NumOps + Copy + Neg<Output = N>>(v: N, g: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let h = height_from_impulse_and_gravity(v, g);
    let t = time_from_impulse_and_gravity(v, g);
    (h, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        let (impulse, gravity) = from_height_and_time(20.0, 10.0);
        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);
    }

    #[test]
    fn test_from_h_v() {
        let (time, gravity) = from_height_and_impulse(20.0, 10.0);
        assert_eq!(time, 4.0);
        assert_eq!(gravity, -2.5);
    }

    #[test]
    fn test_from_h_g() {
        let (time, impulse) = from_height_and_gravity::<f32, f32>(20.0, 10.0);
        assert_eq!(time, 2.0);
        assert_eq!(impulse, 20.0);
    }

    #[test]
    fn test_from_t_v() {
        let (height, gravity) = from_time_and_impulse(10.0, 20.0);
        assert_eq!(height, 100.0);
        assert_eq!(gravity, -2.0);
    }

    #[test]
    fn test_from_t_g() {
        let (height, impulse) = from_time_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(impulse, 10.0);
    }

    #[test]
    fn test_from_v_g() {
        let (height, time) = from_impulse_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(time, 10.0);
    }
}
