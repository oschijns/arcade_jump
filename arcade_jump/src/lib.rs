#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::similar_names)]

/// Trajectory config
pub mod trajectory;

// Contains functions to resolve the value of a parameter given two other parameters
pub mod resolver;
