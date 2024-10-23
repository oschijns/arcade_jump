use crate::{gravity::*, height::*, impulse::*, time::*};
use core::ops::Neg;
use num::{cast::AsPrimitive, traits::NumOps, Float, Zero};

/// Compute the vertical impulse and the gravity from the peak height and the time to reach the peak
#[inline]
pub fn from_height_and_time<N: 'static + NumOps + Copy + Zero + Default>(h: N, t: N) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let v = impulse_from_height_and_time(h, t);
    let g = gravity_from_height_and_time(h, t);
    (v, g)
}

/// Compute the time to reach the peak and the gravity from the peak height and the vertical impulse
#[inline]
pub fn from_height_and_impulse<N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>>(
    h: N,
    v: N,
) -> (N, N)
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
    N: 'static + NumOps + Copy + Zero + Default + AsPrimitive<F>,
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
pub fn from_time_and_impulse<N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>>(
    t: N,
    v: N,
) -> (N, N)
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
pub fn from_impulse_and_gravity<N: 'static + NumOps + Copy + Zero + Default + Neg<Output = N>>(
    v: N,
    g: N,
) -> (N, N)
where
    isize: AsPrimitive<N>,
{
    let h = height_from_impulse_and_gravity(v, g);
    let t = time_from_impulse_and_gravity(v, g);
    (h, t)
}

#[macro_export]
macro_rules! solve {
    ({$height:expr, $time:expr, ?, ?} as $typ:ty) => {
        $crate::solve::from_height_and_time(($height) as $typ, ($time) as $typ)
    };
    ({$height:expr, ?, $impulse:expr, ?} as $typ:ty) => {
        $crate::solve::from_height_and_impulse(($height) as $typ, ($impulse) as $typ)
    };
    ({$height:expr, ?, ?, $gravity:expr} as $typ:ty) => {
        $crate::solve::from_height_and_gravity::<$typ, f64>(($height) as $typ, ($gravity) as $typ)
    };
    ({?, $time:expr, $impulse:expr, ?} as $typ:ty) => {
        $crate::solve::from_time_and_impulse(($time) as $typ, ($impulse) as $typ)
    };
    ({?, $time:expr, ?, $gravity:expr} as $typ:ty) => {
        $crate::solve::from_time_and_gravity(($time) as $typ, ($gravity) as $typ)
    };
    ({?, ?, $impulse:expr, $gravity:expr} as $typ:ty) => {
        $crate::solve::from_impulse_and_gravity(($impulse) as $typ, ($gravity) as $typ)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_h_t() {
        let (impulse, gravity) = from_height_and_time(20.0, 10.0);
        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);

        let (impulse, gravity) = solve![{20.0, 10.0, ?, ?} as f32];
        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);
    }

    #[test]
    fn test_from_h_v() {
        let (time, gravity) = from_height_and_impulse(20.0, 10.0);
        assert_eq!(time, 4.0);
        assert_eq!(gravity, -2.5);

        let (time, gravity) = solve![{20.0, ?, 10.0, ?} as f32];
        assert_eq!(time, 4.0);
        assert_eq!(gravity, -2.5);
    }

    #[test]
    fn test_from_h_g() {
        let (time, impulse) = from_height_and_gravity::<f32, f32>(20.0, 10.0);
        assert_eq!(time, 2.0);
        assert_eq!(impulse, 20.0);

        let (time, impulse) = solve![{20.0, ?, ?, 10.0} as f32];
        assert_eq!(time, 2.0);
        assert_eq!(impulse, 20.0);
    }

    #[test]
    fn test_from_t_v() {
        let (height, gravity) = from_time_and_impulse(10.0, 20.0);
        assert_eq!(height, 100.0);
        assert_eq!(gravity, -2.0);

        let (height, gravity) = solve![{?, 10.0, 20.0, ?} as f32];
        assert_eq!(height, 100.0);
        assert_eq!(gravity, -2.0);
    }

    #[test]
    fn test_from_t_g() {
        let (height, impulse) = from_time_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(impulse, 10.0);

        let (height, impulse) = solve![{?, 10.0, ?, -1.0} as f32];
        assert_eq!(height, 50.0);
        assert_eq!(impulse, 10.0);
    }

    #[test]
    fn test_from_v_g() {
        let (height, time) = from_impulse_and_gravity(10.0, -1.0);
        assert_eq!(height, 50.0);
        assert_eq!(time, 10.0);

        let (height, time) = solve![{?, ?, 10.0, -1.0} as f32];
        assert_eq!(height, 50.0);
        assert_eq!(time, 10.0);
    }
}
