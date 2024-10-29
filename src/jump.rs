use crate::{
    gravity::{gravity_from_height_and_impulse, gravity_from_height_and_time},
    horizontal::from_speed_range_and_ratio,
    solve::from_height_and_time,
};
use num::{cast::AsPrimitive, Float, Zero};

/// Configuration for handling jumps with height control
pub struct JumpTrajectory<N> {
    /// Initial vertical impulse
    main_impulse: N,

    /// Gravity to apply when ascending
    main_gravity_ascend: N,

    /// Gravity to apply when descending
    main_gravity_descend: N,

    /// Gravity to apply when ascending in a small jump
    small_gravity_ascend: N,

    /// Gravity to apply when descending in a small jump
    small_gravity_descend: N,
}

impl<N> JumpTrajectory<N> {
    /// Create a new jump trajectory configuration
    pub fn new<F: Float + Zero + Default + AsPrimitive<F> + AsPrimitive<N>>(
        peak_height: N,
        range: N,
        speed: N,
        offset_ratio: F,
        smalljump_height: N,
    ) -> Self
    where
        isize: AsPrimitive<N> + AsPrimitive<F>,
        N: Zero + AsPrimitive<F>,
    {
        let speed: F = speed.as_();
        let height: F = peak_height.as_();
        let time = from_speed_range_and_ratio(speed, range.as_(), offset_ratio);
        let (main_impulse, main_gravity_ascend) = from_height_and_time::<F>(height, time.0);
        let main_gravity_descend = gravity_from_height_and_time(height, time.1);

        let small_gravity_ascend =
            gravity_from_height_and_impulse(smalljump_height.as_(), main_impulse);
        let small_gravity_descend = if main_gravity_descend < small_gravity_ascend {
            main_gravity_descend
        } else {
            small_gravity_ascend
        };

        Self {
            main_impulse: main_impulse.as_(),
            main_gravity_ascend: main_gravity_ascend.as_(),
            main_gravity_descend: main_gravity_descend.as_(),
            small_gravity_ascend: small_gravity_ascend.as_(),
            small_gravity_descend: small_gravity_descend.as_(),
        }
    }
}

impl<N: Copy> JumpTrajectory<N> {
    /// Get the initial vertical impulse
    #[inline]
    pub fn get_impulse(&self) -> N {
        self.main_impulse
    }

    /// Get the gravity strength
    pub fn get_gravity(&self, holding: bool, ascending: bool) -> N {
        if holding {
            if ascending {
                self.main_gravity_ascend
            } else {
                self.main_gravity_descend
            }
        } else if ascending {
            self.small_gravity_ascend
        } else {
            self.small_gravity_descend
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory() {
        let jump = JumpTrajectory::new(20.0, 20.0, 10.0, 0.6, 10.0, 10.0, 10.0);

        assert_eq!(jump.get_impulse().floor(), 33.0);
        assert_eq!(jump.get_gravity(true, true).floor(), -28.0); // hold + ascend
        assert_eq!(jump.get_gravity(true, false).floor(), -63.0); // hold + descend
        assert_eq!(jump.get_gravity(false, true).floor(), -56.0); // small + ascend
        assert_eq!(jump.get_gravity(false, false).floor(), -63.0); // small + descend

        let jump = JumpTrajectory::new(20, 20, 10, 0.6, 10, 10, 10);

        assert_eq!(jump.get_impulse(), 33);
        assert_eq!(jump.get_gravity(true, true), -27); // hold + ascend
        assert_eq!(jump.get_gravity(true, false), -62); // hold + descend
        assert_eq!(jump.get_gravity(false, true), -55); // small + ascend
        assert_eq!(jump.get_gravity(false, false), -62); // small + descend
    }
}
