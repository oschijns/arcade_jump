//! Computation functions that cannot fail since they do not involve a division

use super::util::*;
use core::ops::{Add, Div, Mul, Neg};
use num_traits::{ConstOne, Float};

/// Compute the peak height from the time to reach the peak and the vertical impulse
#[inline(always)]
pub fn height_from_time_and_impulse<N>(time: N, impulse: N) -> N
where
    N: ConstOne + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    halve(impulse * time)
}

/// Compute the peak height from the time to reach the peak and the gravity
#[inline(always)]
pub fn height_from_time_and_gravity<N>(time: N, gravity: N) -> N
where
    N: Copy + ConstOne + Neg<Output = N> + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    -halve(gravity * pow2(time))
}

/// Compute the vertical impulse from the peak height and the gravity
#[inline(always)]
pub fn impulse_from_height_and_gravity<N>(height: N, gravity: N) -> N
where
    N: Float,
{
    (double(height) * gravity).abs().sqrt()
}

/// Compute the vertical impulse from the time to reach the peak and the gravity
#[inline(always)]
pub fn impulse_from_time_and_gravity<N>(time: N, gravity: N) -> N
where
    N: Neg<Output = N> + Mul<Output = N>,
{
    -gravity * time
}
