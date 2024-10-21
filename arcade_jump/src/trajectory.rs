use crate::resolver::*;
use core::ops::{Add, Div, Mul, Neg};
use num_traits::{ConstOne, Float, Zero};

/// Represents a trajectory of a jump.
#[derive(Debug, Clone, Copy)]
pub struct Trajectory<N> {
    /// The height of the peak of the jump
    height: N,

    /// The time it takes to reach the peak of the jump
    time: N,

    /// The initial impulse applied to the jump
    impulse: N,

    /// The acceleration due to gravity
    gravity: N,
}

impl<N: Copy> Trajectory<N> {
    /// Get the height of the peak
    #[inline]
    pub fn height(&self) -> N {
        self.height
    }

    /// Get the time it takes to reach the peak of the jump
    #[inline]
    pub fn time(&self) -> N {
        self.time
    }

    /// Get the initial impulse applied to the jump
    #[inline]
    pub fn impulse(&self) -> N {
        self.impulse
    }

    /// Get the acceleration due to gravity
    #[inline]
    pub fn gravity(&self) -> N {
        self.gravity
    }
}

impl<N> Trajectory<N>
where
    N: Copy + Zero + Neg<Output = N> + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    /// Construct a trajectory from the height of the peak and the time to reach that peak
    pub fn from_height_and_time(height: N, time: N) -> Result<Self, ResolveError> {
        let impulse = impulse_from_height_and_time(height, time)?;
        let gravity = gravity_from_height_and_time(height, time)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}

impl<N> Trajectory<N>
where
    N: Copy
        + Zero
        + ConstOne
        + Neg<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + Div<Output = N>,
{
    /// Construct a trajectory from the height of the peak and the initial impulse
    pub fn from_height_and_impulse(height: N, impulse: N) -> Result<Self, ResolveError> {
        let time = time_from_height_and_impulse(height, impulse)?;
        let gravity = gravity_from_height_and_impulse(height, impulse)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}

impl<N> Trajectory<N>
where
    N: Zero + Float + Div<Output = N>,
{
    /// Construct a trajectory from the height of the gravity
    pub fn from_height_and_gravity(height: N, gravity: N) -> Result<Self, ResolveError> {
        let time = time_from_height_and_gravity(height, gravity)?;
        let impulse = impulse_from_height_and_gravity(height, gravity)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}

impl<N> Trajectory<N>
where
    N: Copy
        + Zero
        + ConstOne
        + Neg<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + Div<Output = N>,
{
    /// Construct a trajectory from the time to reach the peak and the initial impulse
    pub fn from_time_and_impulse(time: N, impulse: N) -> Result<Self, ResolveError> {
        let height = height_from_time_and_impulse(time, impulse)?;
        let gravity = gravity_from_time_and_impulse(time, impulse)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}

impl<N> Trajectory<N>
where
    N: Copy + ConstOne + Neg<Output = N> + Add<Output = N> + Mul<Output = N> + Div<Output = N>,
{
    /// Construct a trajectory from the time to reach the peak and the gravity
    pub fn from_time_and_gravity(time: N, gravity: N) -> Result<Self, ResolveError> {
        let height = height_from_time_and_gravity(time, gravity)?;
        let impulse = impulse_from_time_and_gravity(time, gravity)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}

impl<N> Trajectory<N>
where
    N: Copy
        + Zero
        + ConstOne
        + Neg<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + Div<Output = N>,
{
    /// Construct a trajectory from the initial impulse and the gravity
    pub fn from_impulse_and_gravity(impulse: N, gravity: N) -> Result<Self, ResolveError> {
        let height = height_from_impulse_and_gravity(impulse, gravity)?;
        let time = time_from_impulse_and_gravity(impulse, gravity)?;
        Ok(Self {
            height,
            time,
            impulse,
            gravity,
        })
    }
}
