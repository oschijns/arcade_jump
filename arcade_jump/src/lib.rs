#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::similar_names)]

/// Trajectory config
pub mod trajectory;

// Contains functions to resolve the value of a parameter given two other parameters
pub mod resolver;

// re-export the compute procedural macro from the macros module
pub mod macros {
    pub use arcade_jump_macros::*;
}
