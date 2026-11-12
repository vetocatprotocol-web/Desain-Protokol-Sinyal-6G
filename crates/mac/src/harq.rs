//! Proses HARQ chase combining per UE.
//!
//! Setiap UE memiliki satu proses HARQ. Paket yang gagal dikirim ulang
//! (chase combining) hingga batas maksimum, kemudian dibuang.

use crate::error::Error;
use alloc::vec::Vec;

/// Batas maksimal retransmisi HARQ (maksimal 4 transmisi total per paket).
///
/// # Example
///
/// ```
/// assert_eq!(sig6g_mac::harq::MAX_HARQ_RETRANSMISSIONS, 3);
/// ```
pub const MAX_HARQ_RETRANSMISSIONS: u8 = 3;

/// Status proses HARQ.
///
/// # Example
///
/// ```
/// use sig6g_mac::harq::HarqState;
/// let s = HarqState::Pending { attempt: 2 };
/// assert!(matches!(s, HarqState::Pending { attempt: 2 }));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarqState {
    /// Tidak ada transmisi aktif.
    Idle,
    /// Tunggu ACK/NACK, percobaan ke-`attempt` (1-based).
    Pending { attempt: u8 },
    /// Melebihi batas retransmisi, paket dibuang.
    Exhausted,
}

/// Hasil operasi transmisi HARQ.
///
/// # Example
///
/// ```
/// use sig6g_mac::harq::HarqResult;
/// let r = HarqResult::Sent { attempt: 1 };
/// assert!(matches!(r, HarqResult::Sent { attempt: 1 }));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarqResult {
    /// Transmisi berhasil diajukan, menunggu ACK/NACK.
    Sent { attempt: u8 },
    /// Paket berhasil diterima (ACK), proses bersih.
    Ack,
    /// Paket gagal (NACK), prosedur berikutnya dijalankan.
    Nack { attempt: u8 },
    /// Paket dibuang karena kehabisan percobaan.
    Dropped,
}

/// Proses HARQ untuk satu UE.
///
/// # Example
///
/// ```
/// use sig6g_mac::harq::{HarqProcess, HarqState};
/// let mut h = HarqProcess::new();
/// assert_eq!(h.state(), HarqState::Idle);
/// h.transmit(b"hello").unwrap();
/// assert_eq!(h.state(), HarqState::Pending { attempt: 1 });
/// h.on_ack();
/// assert_eq!(h.state(), HarqState::Idle);
/// ```
pub struct HarqProcess {
    state: HarqState,
    /// Paket terakhir yang ditransmisikan (untuk chase combining).
    packet: Option<Vec<u8>>,
}

impl HarqProcess {
    /// Buat proses HARQ baru dalam keadaan Idle.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: HarqState::Idle,
            packet: None,
        }
    }

    /// Status proses HARQ saat ini.
    #[must_use]
    pub const fn state(&self) -> HarqState {
        self.state
    }

    /// Mulai transmisi paket baru. Mengembalikan error jika proses tidak Idle.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`Error::HarqExhausted`] jika proses dalam keadaan
    /// `Exhausted` dan harus direset terlebih dahulu.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::harq::{HarqProcess, HarqResult};
    /// let mut h = HarqProcess::new();
    /// let r = h.transmit(b"packet").unwrap();
    /// assert_eq!(r, HarqResult::Sent { attempt: 1 });
    /// ```
    pub fn transmit(&mut self, packet: &[u8]) -> Result<HarqResult, Error> {
        match self.state {
            HarqState::Idle => {
                self.packet = Some(packet.to_vec());
                self.state = HarqState::Pending { attempt: 1 };
                Ok(HarqResult::Sent { attempt: 1 })
            }
            HarqState::Pending { .. } | HarqState::Exhausted => Err(Error::HarqExhausted),
        }
    }

    /// Retransmisikan paket terakhir (chase combining).
    /// Hanya valid ketika state `Pending`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`Error::HarqEmpty`] jika tidak ada paket untuk
    /// diretransmisikan, atau [`Error::HarqExhausted`] jika batas telah
    /// tercapai.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::harq::{HarqProcess, HarqResult};
    /// let mut h = HarqProcess::new();
    /// h.transmit(b"packet").unwrap();
    /// let r = h.retransmit().unwrap();
    /// assert_eq!(r, HarqResult::Nack { attempt: 2 });
    /// ```
    pub fn retransmit(&mut self) -> Result<HarqResult, Error> {
        match self.state {
            HarqState::Pending { attempt } => {
                if attempt > MAX_HARQ_RETRANSMISSIONS {
                    self.state = HarqState::Exhausted;
                    self.packet = None;
                    return Ok(HarqResult::Dropped);
                }
                let next = attempt + 1;
                self.state = HarqState::Pending { attempt: next };
                Ok(HarqResult::Nack { attempt: next })
            }
            HarqState::Idle => Err(Error::HarqEmpty),
            HarqState::Exhausted => Err(Error::HarqExhausted),
        }
    }

    /// Tandai paket sebagai diterima (ACK), bersihkan proses.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::harq::{HarqProcess, HarqState};
    /// let mut h = HarqProcess::new();
    /// h.transmit(b"packet").unwrap();
    /// h.on_ack();
    /// assert_eq!(h.state(), HarqState::Idle);
    /// ```
    pub fn on_ack(&mut self) {
        self.state = HarqState::Idle;
        self.packet = None;
    }

    /// Tandai paket sebagai gagal (NACK), jalankan prosedur berikutnya.
    ///
    /// Mengembalikan `true` jika proses masih aktif, `false` jika dibuang.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::harq::{HarqProcess, HarqState};
    /// let mut h = HarqProcess::new();
    /// h.transmit(b"packet").unwrap();
    /// assert!(h.on_nack());
    /// assert_eq!(h.state(), HarqState::Pending { attempt: 2 });
    /// ```
    pub fn on_nack(&mut self) -> bool {
        match self.state {
            HarqState::Pending { attempt } => {
                if attempt > MAX_HARQ_RETRANSMISSIONS {
                    self.state = HarqState::Exhausted;
                    self.packet = None;
                    false
                } else {
                    let next = attempt + 1;
                    self.state = HarqState::Pending { attempt: next };
                    true
                }
            }
            _ => false,
        }
    }

    /// Reset proses ke keadaan Idle (paket dibuang tanpa ACK).
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::harq::{HarqProcess, HarqState};
    /// let mut h = HarqProcess::new();
    /// h.transmit(b"packet").unwrap();
    /// h.reset();
    /// assert_eq!(h.state(), HarqState::Idle);
    /// ```
    pub fn reset(&mut self) {
        self.state = HarqState::Idle;
        self.packet = None;
    }
}

impl Default for HarqProcess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmit_then_ack() {
        let mut h = HarqProcess::new();
        assert_eq!(h.state(), HarqState::Idle);
        let r = h.transmit(b"hello").unwrap();
        assert_eq!(r, HarqResult::Sent { attempt: 1 });
        assert_eq!(h.state(), HarqState::Pending { attempt: 1 });
        h.on_ack();
        assert_eq!(h.state(), HarqState::Idle);
    }

    #[test]
    fn harq_exhaustion() {
        let mut h = HarqProcess::new();
        h.transmit(b"x").unwrap();
        // attempt 1 -> NACK -> attempt 2
        assert!(h.on_nack());
        assert_eq!(h.state(), HarqState::Pending { attempt: 2 });
        // attempt 2 -> NACK -> attempt 3
        assert!(h.on_nack());
        assert_eq!(h.state(), HarqState::Pending { attempt: 3 });
        // attempt 3 -> NACK -> attempt 4 (MAX+1)
        assert!(h.on_nack());
        assert_eq!(h.state(), HarqState::Pending { attempt: 4 });
        // attempt 4 -> NACK -> Exhausted (Dropped)
        assert!(!h.on_nack());
        assert_eq!(h.state(), HarqState::Exhausted);
    }

    #[test]
    fn retransmit_after_exhaustion_errors() {
        let mut h = HarqProcess::new();
        h.transmit(b"x").unwrap();
        for _ in 0..=MAX_HARQ_RETRANSMISSIONS {
            h.on_nack();
        }
        assert_eq!(h.state(), HarqState::Exhausted);
        assert!(matches!(h.retransmit(), Err(Error::HarqExhausted)));
    }
}
