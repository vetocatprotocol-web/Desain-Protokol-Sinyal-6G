//! Contiguous, zero-copy buffer types for frames and slots.
//!
//! Buffers live in contiguous memory (`SoA` where useful) and expose slices
//! (`&[u8]`, `&[Complex32]`) for borrowing through the pipeline. Ownership is
//! moved between pipeline stages rather than copied.

#![allow(clippy::module_name_repetitions)]

use core::fmt;
use core::ops::{Deref, DerefMut};

/// A plain complex sample (32-bit real + 32-bit imaginary).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Complex32 {
    pub re: i32,
    pub im: i32,
}

impl Complex32 {
    #[must_use]
    pub const fn new(re: i32, im: i32) -> Self {
        Self { re, im }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self { re: 0, im: 0 }
    }
}

impl Default for Complex32 {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Complex32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

/// A contiguous buffer of samples with a fixed capacity known at compile time.
///
/// The buffer stores elements inline (`[T; CAP]`) and tracks a logical length.
/// No heap allocation occurs after construction.
///
/// # Type parameters
/// - `T` — element type.
/// - `CAP` — total capacity (compile time).
#[derive(Debug, Clone)]
pub struct FixedBuffer<T, const CAP: usize> {
    data: [T; CAP],
    len: usize,
}

impl<T, const CAP: usize> FixedBuffer<T, CAP> {
    /// Creates an empty buffer filled with the default value of `T`.
    /// No heap allocation occurs.
    #[must_use]
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self {
            data: [T::default(); CAP],
            len: 0,
        }
    }

    /// Returns the number of valid elements.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns total capacity.
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns a slice of the valid elements.
    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    /// Returns a mutable slice of the valid elements.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.len]
    }

    /// Pushes an element. Returns an error if the buffer is full.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BufferFull`] when the buffer is at capacity.
    pub fn push(&mut self, value: T) -> Result<(), Error> {
        if self.len >= CAP {
            return Err(Error::BufferFull);
        }
        self.data[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /// Clears the logical contents without dropping elements (caller must ensure
    /// `T: Copy` or handle drop semantics).
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T: Default + Copy, const CAP: usize> Default for FixedBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug, const CAP: usize> Deref for FixedBuffer<T, CAP> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: fmt::Debug, const CAP: usize> DerefMut for FixedBuffer<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

/// A slot buffer of complex samples (`[Complex32; CAP]`).
pub type SlotBuffer<const CAP: usize> = FixedBuffer<Complex32, CAP>;

/// A byte payload buffer.
pub type PayloadBuffer<const CAP: usize> = FixedBuffer<u8, CAP>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    BufferFull,
    BufferEmpty,
    OutOfBounds,
    InvalidFormat,
    InvalidState,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferFull => write!(f, "buffer full"),
            Self::BufferEmpty => write!(f, "buffer empty"),
            Self::OutOfBounds => write!(f, "out of bounds"),
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidState => write!(f, "invalid state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_buffer_push_and_slice() {
        let mut buf: SlotBuffer<4> = SlotBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.capacity(), 4);

        buf.push(Complex32::new(1, 2)).unwrap();
        buf.push(Complex32::new(3, 4)).unwrap();
        assert_eq!(buf.len(), 2);
        assert_eq!(
            buf.as_slice(),
            &[Complex32::new(1, 2), Complex32::new(3, 4)]
        );
    }

    #[test]
    fn fixed_buffer_full_errors() {
        let mut buf: SlotBuffer<2> = SlotBuffer::new();
        buf.push(Complex32::zero()).unwrap();
        buf.push(Complex32::zero()).unwrap();
        assert!(matches!(
            buf.push(Complex32::zero()),
            Err(Error::BufferFull)
        ));
    }

    #[test]
    fn fixed_buffer_clear() {
        let mut buf: SlotBuffer<4> = SlotBuffer::new();
        buf.push(Complex32::new(1, 1)).unwrap();
        buf.clear();
        assert!(buf.is_empty());
    }

    #[test]
    fn complex32_default_zero() {
        assert_eq!(Complex32::default(), Complex32::zero());
    }
}
