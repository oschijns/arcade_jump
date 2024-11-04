use crate::math::pow2;
use const_soft_float::soft_f64::SoftF64;

/// Compute the peak height from the time to reach the peak and the vertical impulse
#[inline]
pub const fn height_from_time_and_impulse(time: f64, impulse: f64) -> f64 {
    0.5 * impulse * time
}

/// Compute the peak height from the time to reach the peak and the gravity
#[inline]
pub const fn height_from_time_and_gravity(time: f64, gravity: f64) -> f64 {
    -0.5 * gravity * pow2![time]
}

/// Compute the peak height from the vertical impulse and the gravity
#[inline]
pub const fn height_from_impulse_and_gravity(impulse: f64, gravity: f64) -> f64 {
    if gravity == 0.0 {
        f64::INFINITY
    } else {
        -0.5 * pow2![impulse] / gravity
    }
}

/// Compute time to reach the peak from the peak height and the vertical impulse
#[inline]
pub const fn time_from_height_and_impulse(height: f64, impulse: f64) -> f64 {
    if impulse == 0.0 {
        f64::INFINITY
    } else {
        2.0 * height / impulse
    }
}

/// Compute time to reach the peak from the peak height and the gravity (constant)
#[inline]
pub const fn time_from_height_and_gravity_const(height: f64, gravity: f64) -> f64 {
    if gravity == 0.0 {
        f64::INFINITY
    } else {
        let f = 2.0 * height / gravity;
        SoftF64(if f >= 0.0 { f } else { -f }).sqrt().to_f64()
    }
}

/// Compute time to reach the peak from the peak height and the gravity
#[inline]
pub fn time_from_height_and_gravity(height: f64, gravity: f64) -> f64 {
    if gravity == 0.0 {
        f64::INFINITY
    } else {
        (2.0 * height / gravity).abs().sqrt()
    }
}

/// Compute time to reach the peak from the vertical impulse and the gravity
#[inline]
pub const fn time_from_impulse_and_gravity(impulse: f64, gravity: f64) -> f64 {
    if gravity == 0.0 {
        f64::INFINITY
    } else {
        -impulse / gravity
    }
}

/// Compute the vertical impulse from the peak height and the time to reach the peak
#[inline]
pub const fn impulse_from_height_and_time(height: f64, time: f64) -> f64 {
    if time == 0.0 {
        f64::INFINITY
    } else {
        2.0 * height / time
    }
}

/// Compute the vertical impulse from the peak height and the gravity (constant)
#[inline]
pub const fn impulse_from_height_and_gravity_const(height: f64, gravity: f64) -> f64 {
    let f = 2.0 * height * gravity;
    SoftF64(if f >= 0.0 { f } else { -f }).sqrt().to_f64()
}

/// Compute the vertical impulse from the peak height and the gravity
#[inline]
pub fn impulse_from_height_and_gravity(height: f64, gravity: f64) -> f64 {
    (2.0 * height * gravity).abs().sqrt()
}

/// Compute the vertical impulse from the time to reach the peak and the gravity
#[inline]
pub const fn impulse_from_time_and_gravity(time: f64, gravity: f64) -> f64 {
    -gravity * time
}

/// Compute the gravity from the peak height and the time to reach the peak
#[inline]
pub const fn gravity_from_height_and_time(height: f64, time: f64) -> f64 {
    if time == 0.0 {
        f64::NEG_INFINITY
    } else {
        -2.0 * height / pow2![time]
    }
}

/// Compute the gravity from the peak height and the vertical impulse
#[inline]
pub const fn gravity_from_height_and_impulse(height: f64, impulse: f64) -> f64 {
    if height == 0.0 {
        f64::NEG_INFINITY
    } else {
        -0.5 * pow2![impulse] / height
    }
}

/// Compute the gravity from the time to reach the peak and the vertical impulse
#[inline]
pub const fn gravity_from_time_and_impulse(time: f64, impulse: f64) -> f64 {
    if time == 0.0 {
        f64::NEG_INFINITY
    } else {
        -impulse / time
    }
}

/// Compute the time to reach the peak from the horizontal speed and the range
#[inline]
pub const fn time_from_speed_and_range(speed: f64, range: f64) -> f64 {
    if speed == 0.0 {
        f64::INFINITY
    } else {
        0.5 * range / speed
    }
}

/// Compute the time to reach the peak from the horizontal speed, the range and an arbitrary ratio
#[inline]
pub const fn time_from_speed_and_range_with_ratio(
    speed: f64,
    range: f64,
    ratio: f64,
) -> (f64, f64) {
    if speed == 0.0 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let time = range / speed;
        (time * ratio, time * (1.0 - ratio))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_jump_parameter() {
        use super::*;

        const HEIGHT: f64 = 20.0;
        const TIME: f64 = 10.0;

        const IMPULSE: f64 = impulse_from_height_and_time(HEIGHT, TIME);
        const GRAVITY: f64 = gravity_from_height_and_time(HEIGHT, TIME);

        const TIME2: f64 = time_from_height_and_gravity_const(20.0, GRAVITY);

        assert_eq!(IMPULSE, 4.0);
        assert_eq!(GRAVITY, -0.4);
        assert_eq!(TIME2, 10.0);
    }
}
