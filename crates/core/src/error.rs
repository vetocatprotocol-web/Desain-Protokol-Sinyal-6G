//! Explicit error type for `sig6g-core`.
//!
//! Manual enum (no `thiserror`) because `core` is `no_std`. Error variants are
//! exported with doc comments and `Display` implementations.

use core::fmt;

/// Errors produced by `sig6g-core`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The requested operation would exceed buffer capacity.
    BufferFull,
    /// The buffer contains no elements.
    BufferEmpty,
    /// An index or offset was outside valid bounds.
    OutOfBounds,
    /// The input data does not match the expected format.
    InvalidFormat,
    /// The object is not in a state permitting the operation.
    InvalidState,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferFull => f.write_str("buffer full"),
            Self::BufferEmpty => f.write_str("buffer empty"),
            Self::OutOfBounds => f.write_str("out of bounds"),
            Self::InvalidFormat => f.write_str("invalid format"),
            Self::InvalidState => f.write_str("invalid state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_variants() {
        assert_eq!(alloc::format!("{}", Error::BufferFull), "buffer full");
        assert_eq!(alloc::format!("{}", Error::InvalidState), "invalid state");
    }

    #[test]
    fn debug_derive() {
        assert_eq!(alloc::format!("{:?}", Error::OutOfBounds), "OutOfBounds");
    }
}
