//! Parser dan serializer frame (zero-copy, fuzz-friendly).
//!
//! Format frame (M2):
//!
//! ```text
//!  +--------+--------+--------+--------+--------+--------+--------+--------+
//!  | magic (2) | ver(1) | type(1) |    length (2)    | seq (2)         |
//!  +--------+--------+--------+--------+--------+--------+--------+--------+
//!  |                    payload (length bytes)                           |
//!  +---------------------------------------------------------------------+
//!  |                    crc16 (2, little-endian)                         |
//!  +---------------------------------------------------------------------+
//! ```

use crate::error::Error;
use alloc::vec::Vec;

/// Magic bytes frame sig6g ("SI").
const MAGIC: [u8; 2] = [0x53, 0x49];
/// Panjang header frame dalam byte.
const HEADER_LEN: usize = 8;
/// Versi format frame yang didukung.
const CURRENT_VERSION: u8 = 1;

/// Frame yang di-parse (payload sebagai slice peminjam, zero-copy).
///
/// # Example
///
/// ```
/// use sig6g_link::frame::Frame;
///
/// let payload: &[u8] = b"data";
/// let frame = Frame { ver: 1, r#type: 0, seq: 3, payload };
/// assert_eq!(frame.seq, 3);
/// assert_eq!(frame.payload, b"data");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame<'a> {
    pub ver: u8,
    pub r#type: u8,
    pub seq: u16,
    pub payload: &'a [u8],
}

/// CRC-16-CCITT (poly 0x1021, init 0xFFFF, tanpa refleksi).
fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// uraikan frame dari `buf`. Mengembalikan frame dan jumlah byte yang
/// dikonsumsi. Payload adalah slice peminjam dari `buf` (zero-copy).
///
/// # Errors
///
/// Mengembalikan [`Error::TooShort`], [`Error::BadMagic`],
/// [`Error::BadVersion`], [`Error::LengthExceeds`], atau
/// [`Error::BadCrc`].
///
/// # Example
///
/// ```
/// use sig6g_link::frame::{parse_frame, serialize_frame};
///
/// let payload = b"hello";
/// let framed = serialize_frame(1, 0, 7, payload);
/// let (frame, consumed) = parse_frame(&framed).unwrap();
/// assert_eq!(consumed, framed.len());
/// assert_eq!(frame.seq, 7);
/// assert_eq!(frame.payload, payload);
/// ```
pub fn parse_frame(buf: &[u8]) -> Result<(Frame<'_>, usize), Error> {
    if buf.len() < HEADER_LEN {
        return Err(Error::TooShort);
    }
    if buf[0] != MAGIC[0] || buf[1] != MAGIC[1] {
        return Err(Error::BadMagic);
    }
    let ver = buf[2];
    if ver != CURRENT_VERSION {
        return Err(Error::BadVersion);
    }
    let r#type = buf[3];
    let length = u16::from_le_bytes([buf[4], buf[5]]) as usize;
    let seq = u16::from_le_bytes([buf[6], buf[7]]);
    if buf.len() < HEADER_LEN + length {
        return Err(Error::LengthExceeds);
    }
    let payload = &buf[HEADER_LEN..HEADER_LEN + length];
    let crc_offset = HEADER_LEN + length;
    if buf.len() < crc_offset + 2 {
        return Err(Error::TooShort);
    }
    let crc = u16::from_le_bytes([buf[crc_offset], buf[crc_offset + 1]]);
    let calc = crc16_ccitt(&buf[..crc_offset]);
    if calc != crc {
        return Err(Error::BadCrc);
    }
    Ok((
        Frame {
            ver,
            r#type,
            seq,
            payload,
        },
        crc_offset + 2,
    ))
}

/// Susun frame ke `Vec<u8>`.
///
/// # Example
///
/// ```
/// use sig6g_link::frame::serialize_frame;
///
/// let framed = serialize_frame(1, 0, 7, b"hello");
/// assert_eq!(framed[0], 0x53); // magic 'S'
/// assert_eq!(framed[1], 0x49); // magic 'I'
/// assert_eq!(framed[2], 1);    // version
/// assert_eq!(framed[3], 0);    // type data
/// ```
#[must_use]
pub fn serialize_frame(ver: u8, r#type: u8, seq: u16, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len() + 2);
    out.extend_from_slice(&MAGIC);
    out.push(ver);
    out.push(r#type);
    out.extend_from_slice(&length_as_bytes(payload.len()));
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(payload);
    let crc = crc16_ccitt(&out);
    out.extend_from_slice(&crc.to_le_bytes());
    out
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
const fn length_as_bytes(len: usize) -> [u8; 2] {
    (len as u16).to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_data_frame() {
        let payload = b"hello sig6g";
        let framed = serialize_frame(1, 0, 42, payload);
        let (frame, consumed) = parse_frame(&framed).unwrap();
        assert_eq!(consumed, framed.len());
        assert_eq!(frame.ver, 1);
        assert_eq!(frame.r#type, 0);
        assert_eq!(frame.seq, 42);
        assert_eq!(frame.payload, payload);
        // Round-trip serialize ulang == byte asli.
        assert_eq!(
            serialize_frame(frame.ver, frame.r#type, frame.seq, frame.payload),
            framed
        );
    }

    #[test]
    fn empty_payload() {
        let framed = serialize_frame(1, 1, 0, &[]);
        let (frame, _) = parse_frame(&framed).unwrap();
        assert!(frame.payload.is_empty());
    }

    #[test]
    fn bad_magic() {
        let mut framed = serialize_frame(1, 0, 1, b"x");
        framed[0] = 0x00;
        assert!(matches!(parse_frame(&framed), Err(Error::BadMagic)));
    }

    #[test]
    fn bad_version() {
        let mut framed = serialize_frame(1, 0, 1, b"x");
        framed[2] = 9;
        assert!(matches!(parse_frame(&framed), Err(Error::BadVersion)));
    }

    #[test]
    fn length_exceeds() {
        let mut framed = serialize_frame(1, 0, 1, b"x");
        // Panjang palsu yang lebih besar dari payload aktual.
        framed[4] = 255;
        framed[5] = 255;
        assert!(matches!(parse_frame(&framed), Err(Error::LengthExceeds)));
    }

    #[test]
    fn bad_crc() {
        let mut framed = serialize_frame(1, 0, 1, b"x");
        let last = framed.len() - 1;
        framed[last] ^= 0xFF;
        assert!(matches!(parse_frame(&framed), Err(Error::BadCrc)));
    }

    #[test]
    fn too_short() {
        assert!(matches!(parse_frame(&[0x53]), Err(Error::TooShort)));
    }

    #[test]
    fn crc_known_vector() {
        // CRC-16-CCITT (init 0xFFFF) dari "123456789" = 0x29B1.
        assert_eq!(crc16_ccitt(b"123456789"), 0x29B1);
    }
}
