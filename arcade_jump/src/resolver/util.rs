//! Simple functions for num-trait

use core::ops::{Add, Div, Mul};
use num_traits::ConstOne;

/// Compute the half of a value
#[inline(always)]
pub(crate) fn halve<N>(n: N) -> N
where
    N: ConstOne + Add<Output = N> + Div<Output = N>,
{
    let two = N::ONE + N::ONE;
    n / two
}

/// Compute the double of a value
#[inline(always)]
pub(crate) fn double<N>(n: N) -> N
where
    N: Copy + Add<Output = N>,
{
    n + n
}

/// Compute the square of a value
#[inline(always)]
pub(crate) fn pow2<N>(n: N) -> N
where
    N: Copy + Mul<Output = N>,
{
    n * n
}
