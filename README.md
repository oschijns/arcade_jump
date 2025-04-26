# Arcade Jump
Simple crate with utilities for creating jump trajectories for video games.


## Introduction

In real-life, a projectile thrown in the air follows a parabolic trajectory.
In video games, we want to reproduce this behavior but with some variation.
We want to be able to control the jump's height by holding or releasing the jump button.
And we want to fall faster than we ascend.

This crate intends to provide a simple and efficient way to create jump trajectories for video games while taking into account all the little edge cases a game developper could think of.
(e.g. double jump, triple jump, wall jump, bouncing off an enemy head, bouncing on a bumper, etc..)

This is possible by composing a trajectory based on the four parameters below:
- The initial vertical velocity (_Impulse_)
- The gravity applied to the object (_Gravity_)
- The height of peak of the jump (_Height_)
- The time to reach the peak of the jump (_Time_)

Each of those four parameters can be computed by picking only two of the others.


## Features

This crate is _no-std_ compatible. It supports the `num-traits` crate.
It provides three different ways to compute the four parameters:
- Using the raw functions from the `resolver` module
- Using the `Trajectory` type
- Using the `compute!` procedural macro


## Usage

### Raw functions

The raw functions for computing each of the four parameters can be found in the `resolver` module.
All of those functions return a `Result<N, ResolveError>` type.
Because some of the computations involve divisions, providing a parameter with a value of zero will result in an error.


### Trajectory type

The crate provide a simple `trajectory::Trajectory<N>` type which contains the four parameters as fields. It is more intended as an example on how to use this crate but still can be used as is if you don't care about computing and storing extra parameters that you may not use.


### Compute macro

Finally, the crate provide a `macros::compute` procedural macro for writing parameters computation in a less convoluted way.
The macro expect you to write the two parameters you want as input and the one or two parameters you expect as output.
The macro will take care of identifying the type of each of the parameters and forwarding computation errors.

```rust
use arcade_jump::{resolver::ResolveError, macros::compute};

fn main() -> Result<(), ResolveError> {
    // I want to define a jump trajectory of 20 units in height and 2 seconds to reach that height.
    let my_height = 20.0f32;
    let my_time = 2.0f32;

    // Let's compute the vertical initial velocity and the gravity.
    // As long as they are properly identified, the order of the parameters does not matter.
    // We can also enforce a numeric type by using the `as` keyword.
    let (my_impulse, my_gravity) = compute!(Height(my_height), Time(my_time) => Impulse, Gravity as f32)?;

    // Let's compute a variation of this jump where the player will reach
    // a height half as high by releasing the jump button earlier.
    // Here the initial vertical velocity is the same for both jump variations,
    // but we define the peak height as being half of the original height.
    // This means that the gravity will be stronger.
    // This is unrealistic but this is how jumps are implemented in most games.
    let my_gravity2 = compute!(Impulse(my_impulse), Height(my_height / 2) => Gravity)?;

    // If we want to implement a double jump, we can pick a new height
    // and the initial gravity and compute a new vertical impulse.
    // This time we use shorthands of the four parameters types.
    let my_impulse2 = compute!(G(my_gravity2), H(my_height / 3) => I)?;
}
```
