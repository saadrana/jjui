//! Custom error type to be used throughout the crate
//!
//! The purpose of this is to map all error types from different crates into one common error type.
//! This allows the use of ? operator making our code simpler

use std::io;
use thiserror::Error;

/// Universal Error enum for the crate
///
/// This will allow the use of ? throughout the crate by mapping Error from external creates to a common type.
#[derive(Error, Debug)]
pub enum Error {
    /// Mapping from std::io::Error to MyError::Io
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Alias for Result (syntactic sugar)
pub type Result<T, E = Error> = std::result::Result<T, E>;
