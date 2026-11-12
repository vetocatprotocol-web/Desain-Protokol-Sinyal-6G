//! # sig6g-link
//!
//! Lapisan link untuk framework protokol sinyal 6G.
//!
//! Menyediakan:
//! - [`state`] — state machine koneksi dengan typestate (`Idle`,
//!   `Acquiring`, `Synced`, `Connected`). State ilegal tidak bisa
//!   dikompilasi.
//! - [`frame`] — parser dan serializer frame zero-copy, fuzz-friendly.
//! - [`error`] — tipe error eksplisit per crate (manual enum).
//!
//! `#![forbid(unsafe_code)]` dan `#![no_std]` dienkode.

#![forbid(unsafe_code)]
#![no_std]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

pub mod error;
pub mod frame;
pub mod state;
