//! # sig6g-mac
//!
//! Lapisan MAC untuk framework protokol sinyal 6G.
//!
//! Menyediakan:
//! - [`MacScheduler`] — penjadwal round-robin multi-UE, deterministik.
//! - [`HarqProcess`] — proses HARQ chase combining per UE dengan batas
//!   retransmisi.
//! - [`HarqState`] — status proses HARQ (`Idle`, `Pending`, `Exhausted`).
//! - [`Error`] — tipe error eksplisit per crate (manual enum).
//!
//! `#![forbid(unsafe_code)]` dan `#![no_std]` dienkode. `std` hanya diperlukan
//! untuk simulasi/bench (crate `sim`), bukan di sini.

#![forbid(unsafe_code)]
#![no_std]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

pub mod error;
pub mod harq;
pub mod scheduler;
