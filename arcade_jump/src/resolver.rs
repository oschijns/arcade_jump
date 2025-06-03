/// Offers computation functions that cannot fail, computation that necessitate a division are not supported
pub mod nofailure;

/// Contains error types
pub mod error;

/// Basic utility functions
pub(crate) mod util;

use core::ops::{Add, Div, Mul, Neg, Sub};
use error::{Error, ErrorTime};
use num_traits::{ConstOne, Float, Zero};
use util::*;

/// Compute the peak height from the time to reach the peak and the vertical impulse
#[inline]
pub fn height_from_time_and_impulse<N>(time: N, impulse: N) -> Result<N, Error>
where
    N: ConstOne + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    Ok(nofailure::height_from_time_and_impulse(time, impulse))
}

/// Compute the peak height from the time to reach the peak and the gravity
#[inline]
pub fn height_from_time_and_gravity<N>(time: N, gravity: N) -> Result<N, Error>
where
    N: Copy + ConstOne + Neg<Output = N> + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    Ok(nofailure::height_from_time_and_gravity(time, gravity))
}

/// Compute the peak height from the vertical impulse and the gravity
#[inline]
pub fn height_from_impulse_and_gravity<N>(impulse: N, gravity: N) -> Result<N, Error>
where
    N: Copy
        + Zero
        + ConstOne
        + Neg<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + Div<Output = N>,
{
    if gravity.is_zero() {
        Err(Error::Gravity)
    } else {
        Ok(-halve(pow2(impulse)) / gravity)
    }
}

/// Compute time to reach the peak from the peak height and the vertical impulse
#[inline]
pub fn time_from_height_and_impulse<N>(height: N, impulse: N) -> Result<N, Error>
where
    N: Copy + Zero + Add<Output = N> + Div<Output = N>,
{
    if impulse.is_zero() {
        Err(Error::Impulse)
    } else {
        Ok(double(height) / impulse)
    }
}
/// Compute time to reach the peak from the peak height and the gravity
#[inline]
pub fn time_from_height_and_gravity<N>(height: N, gravity: N) -> Result<N, Error>
where
    N: Zero + Float + Div<Output = N>,
{
    if gravity.is_zero() {
        Err(Error::Gravity)
    } else {
        Ok((double(height) / gravity).abs().sqrt())
    }
}

/// Compute time to reach the peak from the vertical impulse and the gravity
#[inline]
pub fn time_from_impulse_and_gravity<N>(impulse: N, gravity: N) -> Result<N, Error>
where
    N: Zero + Neg<Output = N> + Div<Output = N>,
{
    if gravity.is_zero() {
        Err(Error::Gravity)
    } else {
        Ok(-impulse / gravity)
    }
}

/// Compute the vertical impulse from the peak height and the time to reach the peak
#[inline]
pub fn impulse_from_height_and_time<N>(height: N, time: N) -> Result<N, Error>
where
    N: Copy + Zero + Add<Output = N> + Div<Output = N>,
{
    if time.is_zero() {
        Err(Error::Time)
    } else {
        Ok(double(height) / time)
    }
}

/// Compute the vertical impulse from the peak height and the gravity
#[inline]
pub fn impulse_from_height_and_gravity<N>(height: N, gravity: N) -> Result<N, Error>
where
    N: Float,
{
    Ok(nofailure::impulse_from_height_and_gravity(height, gravity))
}

/// Compute the vertical impulse from the time to reach the peak and the gravity
#[inline]
pub fn impulse_from_time_and_gravity<N>(time: N, gravity: N) -> Result<N, Error>
where
    N: Neg<Output = N> + Mul<Output = N>,
{
    Ok(nofailure::impulse_from_time_and_gravity(time, gravity))
}

/// Compute the gravity from the peak height and the time to reach the peak
#[inline]
pub fn gravity_from_height_and_time<N>(height: N, time: N) -> Result<N, Error>
where
    N: Copy + Zero + Neg<Output = N> + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    if time.is_zero() {
        Err(Error::Time)
    } else {
        Ok(-double(height) / pow2(time))
    }
}

/// Compute the gravity from the peak height and the vertical impulse
#[inline]
pub fn gravity_from_height_and_impulse<N>(height: N, impulse: N) -> Result<N, Error>
where
    N: Copy
        + Zero
        + ConstOne
        + Neg<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + Div<Output = N>,
{
    if height.is_zero() {
        Err(Error::Height)
    } else {
        Ok(-halve(pow2(impulse)) / height)
    }
}

/// Compute the gravity from the time to reach the peak and the vertical impulse
#[inline]
pub fn gravity_from_time_and_impulse<N>(time: N, impulse: N) -> Result<N, Error>
where
    N: Zero + Neg<Output = N> + Div<Output = N>,
{
    if time.is_zero() {
        Err(Error::Time)
    } else {
        Ok(-impulse / time)
    }
}

/// Compute the time to reach the peak from the horizontal speed and the range
#[inline]
pub fn time_from_speed_and_range<N>(speed: N, range: N) -> Result<N, ErrorTime>
where
    N: Zero + ConstOne + Add<Output = N> + Div<Output = N> + Div<Output = N>,
{
    if speed.is_zero() {
        Err(ErrorTime::Speed)
    } else {
        Ok(halve(range) / speed)
    }
}

/// Compute the time to reach the peak from the horizontal speed, the range and an arbitrary ratio
#[inline]
pub fn time_from_speed_and_range_with_ratio<N>(
    speed: N,
    range: N,
    ratio: N,
) -> Result<(N, N), ErrorTime>
where
    N: Copy
        + Zero
        + ConstOne
        + Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Div<Output = N>,
{
    if speed.is_zero() {
        Err(ErrorTime::Speed)
    } else {
        let time = halve(range) / speed;
        Ok((time * ratio, time * (N::ONE - ratio)))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_jump_parameter() {
        use super::*;

        const HEIGHT: f32 = 20.0;
        const TIME: f32 = 10.0;

        let impulse = impulse_from_height_and_time::<f32>(HEIGHT, TIME).unwrap();
        let gravity: f32 = gravity_from_height_and_time(HEIGHT, TIME).unwrap();
        let time2: f32 = time_from_height_and_gravity(20.0, gravity).unwrap();

        assert_eq!(impulse, 4.0);
        assert_eq!(gravity, -0.4);
        assert_eq!(time2, 10.0);
    }
}
