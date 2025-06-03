//! Error types

/// Specify the error encountered when resolving the parameters
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Height of the peak cannot be null")]
    Height,

    #[error("Time to reach the peak cannot be null")]
    Time,

    #[error("Initial vertical impulse cannot be null")]
    Impulse,

    #[error("Gravity cannot be null")]
    Gravity,
}

/// Specify the error encountered when resolving the parameters to
/// compute the time to reach the peak from horizontal range and speed.
#[derive(Debug, thiserror::Error)]
pub enum ErrorTime {
    #[error("Time to reach the distance cannot be null")]
    Time,

    #[error("Distance cannot be null")]
    Range,

    #[error("Horizontal speed cannot be null")]
    Speed,
}

/// Trivially convert an ErrorTime into Error::Time at the cost of
/// losing information about the parameter that caused the error.
impl From<ErrorTime> for Error {
    fn from(_error: ErrorTime) -> Self {
        Self::Time
    }
}
