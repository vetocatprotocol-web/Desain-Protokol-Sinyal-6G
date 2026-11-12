//! Penjadwal round-robin multi-UE.
//!
//! Tiap slot dipilih paling banyak satu UE. Urutan selalu deterministik
//! untuk `(slot, active_ues)` yang sama.

use crate::error::Error;

/// Jumlah UE terdaftar maksimum.
///
/// # Example
///
/// ```
/// assert_eq!(sig6g_mac::scheduler::MAX_UE, 64);
/// ```
pub const MAX_UE: usize = 64;

/// Penjadwal round-robin multi-UE.
///
/// Menjaga daftar UE terdaftar dan indeks slot terakhir yang dijadwalkan
/// untuk setiap UE (untuk kepadalan keadilan).
///
/// # Example
///
/// ```
/// use sig6g_mac::scheduler::MacScheduler;
/// let mut sched = MacScheduler::new();
/// sched.register_ue(1).unwrap();
/// assert_eq!(sched.schedule(0, &[1]).unwrap(), 1);
/// ```
pub struct MacScheduler {
    ues: alloc::vec::Vec<u16>,
    /// Indeks UE berikutnya dalam daftar terdaftar (round-robin pointer).
    next_idx: usize,
}

impl MacScheduler {
    /// Buat penjadwal baru tanpa terdaftar UE.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let sched = MacScheduler::new();
    /// assert!(sched.registered_ues().is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ues: alloc::vec::Vec::new(),
            next_idx: 0,
        }
    }

    /// Daftarkan UE. Mengembalikan error jika melebihi [`MAX_UE`] atau UE
    /// sudah terdaftar.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`Error::UeLimitExceeded`] jika jumlah UE terdaftar
    /// sudah mencapai `MAX_UE`.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let mut sched = MacScheduler::new();
    /// assert!(sched.register_ue(1).is_ok());
    /// assert!(sched.register_ue(1).is_err());
    /// ```
    pub fn register_ue(&mut self, ue: u16) -> Result<(), Error> {
        if self.ues.len() >= MAX_UE {
            return Err(Error::UeLimitExceeded);
        }
        if self.ues.contains(&ue) {
            return Err(Error::UeNotFound);
        }
        self.ues.push(ue);
        self.ues.sort_unstable();
        Ok(())
    }

    /// Hapus pendaftaran UE.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let mut sched = MacScheduler::new();
    /// sched.register_ue(1).unwrap();
    /// sched.deregister_ue(1);
    /// assert!(sched.registered_ues().is_empty());
    /// ```
    pub fn deregister_ue(&mut self, ue: u16) {
        if let Some(pos) = self.ues.iter().position(|&u| u == ue) {
            self.ues.remove(pos);
            if self.next_idx >= self.ues.len() {
                self.next_idx = 0;
            }
        }
    }

    /// Daftar UE yang terdaftar (urutan terurut).
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let mut sched = MacScheduler::new();
    /// sched.register_ue(3).unwrap();
    /// sched.register_ue(1).unwrap();
    /// assert_eq!(sched.registered_ues(), &[1, 3]);
    /// ```
    #[must_use]
    pub fn registered_ues(&self) -> &[u16] {
        &self.ues
    }

    /// Pilih UE yang akan dilayani pada `slot` dari `active_ues` (subset
    /// dari UE terdaftar yang aktif pada slot ini).
    ///
    /// Mengembalikan `None` jika tidak ada UE aktif.
    ///
    /// # Errors
    ///
    /// Tidak mengembalikan error; `None` digunakan untuk tidak ada UE aktif.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let mut sched = MacScheduler::new();
    /// sched.register_ue(1).unwrap();
    /// sched.register_ue(2).unwrap();
    /// sched.register_ue(3).unwrap();
    /// let active = [1u16, 2, 3];
    /// let picked: Vec<u16> = (0..6)
    ///     .map(|s| sched.schedule(s, &active).unwrap())
    ///     .collect();
    /// assert_eq!(picked, vec![1, 2, 3, 1, 2, 3]);
    /// ```
    pub fn schedule(&mut self, _slot: usize, active_ues: &[u16]) -> Option<u16> {
        if active_ues.is_empty() || self.ues.is_empty() {
            return None;
        }

        // Round-robin: cari UE aktif berikutnya mulai dari next_idx,
        // putar-seputar melalui daftar terdaftar.
        let n = self.ues.len();
        for offset in 0..n {
            let idx = (self.next_idx + offset) % n;
            let ue = self.ues[idx];
            if active_ues.contains(&ue) {
                self.next_idx = (idx + 1) % n;
                return Some(ue);
            }
        }

        None
    }

    /// Pilih UE tanpa memutar pointer (peek).
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_mac::scheduler::MacScheduler;
    /// let mut sched = MacScheduler::new();
    /// sched.register_ue(5).unwrap();
    /// assert_eq!(sched.peek_schedule(0, &[5]).unwrap(), 5);
    /// // Pointer tidak berputar saat peek.
    /// assert_eq!(sched.schedule(0, &[5]).unwrap(), 5);
    /// ```
    #[must_use]
    pub fn peek_schedule(&self, _slot: usize, active_ues: &[u16]) -> Option<u16> {
        if active_ues.is_empty() || self.ues.is_empty() {
            return None;
        }
        let n = self.ues.len();
        for offset in 0..n {
            let idx = (self.next_idx + offset) % n;
            let ue = self.ues[idx];
            if active_ues.contains(&ue) {
                return Some(ue);
            }
        }
        None
    }
}

impl Default for MacScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_robin_exclusivity() {
        let mut sched = MacScheduler::new();
        sched.register_ue(1).unwrap();
        sched.register_ue(2).unwrap();
        sched.register_ue(3).unwrap();

        let active = [1u16, 2, 3];
        let mut chosen = alloc::vec::Vec::new();
        for slot in 0..9 {
            let ue = sched.schedule(slot, &active).unwrap();
            chosen.push(ue);
        }
        // Setiap slot paling banyak satu UE (trivial karena schedule mengembalikan satu).
        assert_eq!(chosen.len(), 9);
        // Round-robin: 1,2,3,1,2,3,...
        assert_eq!(chosen, alloc::vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn no_active_ue_returns_none() {
        let mut sched = MacScheduler::new();
        sched.register_ue(1).unwrap();
        assert_eq!(sched.schedule(0, &[]), None);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn ue_limit() {
        let mut sched = MacScheduler::new();
        for i in 0..MAX_UE as u16 {
            assert!(sched.register_ue(i).is_ok());
        }
        assert!(matches!(
            sched.register_ue(MAX_UE as u16),
            Err(Error::UeLimitExceeded)
        ));
    }

    #[test]
    fn duplicate_ue_errors() {
        let mut sched = MacScheduler::new();
        sched.register_ue(5).unwrap();
        assert!(matches!(sched.register_ue(5), Err(Error::UeNotFound)));
    }
}
