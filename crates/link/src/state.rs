//! State machine koneksi dengan typestate.
//!
//! Setiap state adalah tipe unit. Transisi hanya melalui metode inherent yang
//! mengembalikan tipe state baru, sehingga transisi ilegal (mis. `connect()`
//! dari `Idle`) tidak terkompilasi.

use core::fmt;

/// Trait state koneksi.
pub trait LinkState {
    /// ID state untuk debugging (tidak dipakai dalam logika transisi).
    fn state_id(&self) -> u8;
}

/// Tidak ada koneksi. Transisi: `Idle -> Acquiring`.
///
/// # Example
///
/// ```
/// use sig6g_link::state::{Idle, LinkState};
///
/// let idle = Idle;
/// assert_eq!(idle.state_id(), 0);
/// let acquiring = idle.acquire();
/// assert_eq!(acquiring.state_id(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Idle;

impl LinkState for Idle {
    fn state_id(&self) -> u8 {
        0
    }
}

impl fmt::Display for Idle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Idle")
    }
}

impl Idle {
    /// Memulai akuisisi sinkronisasi.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_link::state::{Idle, LinkState};
    ///
    /// let idle = Idle;
    /// let acquiring = idle.acquire();
    /// assert_eq!(acquiring.state_id(), 1);
    /// ```
    #[must_use]
    pub const fn acquire(self) -> Acquiring {
        Acquiring
    }
}

/// Mencari sinkronisasi frame/pilot. Transisi: `Acquiring -> Synced`.
///
/// # Example
///
/// ```
/// use sig6g_link::state::{Acquiring, LinkState};
///
/// let acquiring = Acquiring;
/// assert_eq!(acquiring.state_id(), 1);
/// let synced = acquiring.sync();
/// assert_eq!(synced.state_id(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Acquiring;

impl LinkState for Acquiring {
    fn state_id(&self) -> u8 {
        1
    }
}

impl fmt::Display for Acquiring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Acquiring")
    }
}

impl Acquiring {
    /// Sinkronisasi frame telah tercapai.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_link::state::{Acquiring, LinkState};
    ///
    /// let acquiring = Acquiring;
    /// let synced = acquiring.sync();
    /// assert_eq!(synced.state_id(), 2);
    /// ```
    #[must_use]
    pub const fn sync(self) -> Synced {
        Synced
    }
}

/// Sinkronisasi frame telah tercapai. Transisi: `Synced -> Connected`.
///
/// # Example
///
/// ```
/// use sig6g_link::state::{Synced, LinkState};
///
/// let synced = Synced;
/// assert_eq!(synced.state_id(), 2);
/// let connected = synced.connect();
/// assert_eq!(connected.state_id(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Synced;

impl LinkState for Synced {
    fn state_id(&self) -> u8 {
        2
    }
}

impl fmt::Display for Synced {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Synced")
    }
}

impl Synced {
    /// Handshake selesai, koneksi siap lalu lintas data.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_link::state::{Synced, LinkState};
    ///
    /// let synced = Synced;
    /// let connected = synced.connect();
    /// assert_eq!(connected.state_id(), 3);
    /// ```
    #[must_use]
    pub const fn connect(self) -> Connected {
        Connected
    }
}

/// Koneksi siap lalu lintas data. Transisi: `Connected -> Idle`.
///
/// # Example
///
/// ```
/// use sig6g_link::state::{Connected, LinkState};
///
/// let connected = Connected;
/// assert_eq!(connected.state_id(), 3);
/// let idle = connected.disconnect();
/// assert_eq!(idle.state_id(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

impl LinkState for Connected {
    fn state_id(&self) -> u8 {
        3
    }
}

impl fmt::Display for Connected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Connected")
    }
}

impl Connected {
    /// Putus koneksi, kembali ke Idle.
    ///
    /// # Example
    ///
    /// ```
    /// use sig6g_link::state::{Connected, LinkState};
    ///
    /// let connected = Connected;
    /// let idle = connected.disconnect();
    /// assert_eq!(idle.state_id(), 0);
    /// ```
    #[must_use]
    pub const fn disconnect(self) -> Idle {
        Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cycle_compile() {
        // Idle -> Acquiring -> Synced -> Connected -> Idle
        let idle = Idle;
        let acquiring = idle.acquire();
        assert_eq!(acquiring.state_id(), 1);
        let synced = acquiring.sync();
        assert_eq!(synced.state_id(), 2);
        let connected = synced.connect();
        assert_eq!(connected.state_id(), 3);
        let idle2 = connected.disconnect();
        assert_eq!(idle2.state_id(), 0);
    }

    #[test]
    fn state_ids_unique() {
        assert_ne!(Idle.state_id(), Acquiring.state_id());
        assert_ne!(Acquiring.state_id(), Synced.state_id());
        assert_ne!(Synced.state_id(), Connected.state_id());
    }

    #[test]
    fn display_states() {
        assert_eq!(alloc::format!("{Idle}"), "Idle");
        assert_eq!(alloc::format!("{Acquiring}"), "Acquiring");
        assert_eq!(alloc::format!("{Synced}"), "Synced");
        assert_eq!(alloc::format!("{Connected}"), "Connected");
    }
}
