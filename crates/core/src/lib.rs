//! # sig6g-core
//!
//! No-std foundation crate for the 6G signal protocol framework.
//! Provides:
//! - [`Numerology`] — compile-time FFT/subcarrier/slot sizing via const generics.
//! - [`FrameBuffer`] / [`SlotBuffer`] — contiguous, zero-copy buffers.
//! - [`Error`] — explicit error type per crate (manual enum, no `thiserror`).
//!
//! `#![forbid(unsafe_code)]` is enforced. No heap allocation on the hot path.

#![forbid(unsafe_code)]
#![no_std]

pub mod buffer;
pub mod error;
pub mod numerology;

#[cfg(test)]
extern crate alloc;
