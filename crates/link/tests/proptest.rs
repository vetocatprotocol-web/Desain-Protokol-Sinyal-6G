//! Property tests untuk `sig6g-link` (10.000 kasus per properti di CI).
//! Properti: typestate transisi valid, frame round-trip bit-identik,
//! CRC deteksi korupsi.

use proptest::prelude::*;

use sig6g_link::frame::{parse_frame, serialize_frame};
use sig6g_link::state::{Acquiring, Connected, Idle, LinkState, Synced};

proptest! {
    #[test]
    fn frame_roundtrip_preserves_payload(
        payload in proptest::collection::vec(any::<u8>(), 0..64),
        r#type in 0u8..16,
        seq in 0u16..1000,
    ) {
        let serialized = serialize_frame(1, r#type, seq, &payload);
        let (parsed, _consumed) = parse_frame(&serialized).unwrap();
        prop_assert_eq!(parsed.r#type, r#type);
        prop_assert_eq!(parsed.seq, seq);
        prop_assert_eq!(parsed.payload, &payload[..]);
    }

    #[test]
    fn corrupted_crc_is_rejected(
        payload in proptest::collection::vec(any::<u8>(), 0..64),
    ) {
        let mut serialized = serialize_frame(1, 1, 0, &payload);
        let last = serialized.len() - 1;
        serialized[last] = serialized[last].wrapping_add(1);
        prop_assert!(parse_frame(&serialized).is_err());
    }
}

#[test]
fn typestate_idle_to_acquiring() {
    let idle = Idle;
    let acquiring = idle.acquire();
    assert_eq!(acquiring.state_id(), 1);
}

#[test]
fn typestate_acquiring_to_synced() {
    let acquiring = Acquiring;
    let synced = acquiring.sync();
    assert_eq!(synced.state_id(), 2);
}

#[test]
fn typestate_synced_to_connected() {
    let synced = Synced;
    let connected = synced.connect();
    assert_eq!(connected.state_id(), 3);
}

#[test]
fn typestate_connected_to_idle() {
    let connected = Connected;
    let back = connected.disconnect();
    assert_eq!(back.state_id(), 0);
}
