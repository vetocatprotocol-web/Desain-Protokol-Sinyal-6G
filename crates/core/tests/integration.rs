//! Integration tests for `sig6g-core`.

use sig6g_core::{buffer::*, error::Error};

#[test]
fn slot_buffer_deref_to_slice() {
    let mut buf: SlotBuffer<3> = SlotBuffer::new();
    buf.push(Complex32::new(7, 8)).unwrap();
    let s: &[Complex32] = &buf;
    assert_eq!(s.len(), 1);
    assert_eq!(s[0], Complex32::new(7, 8));
}

#[test]
const fn error_is_send_sync() {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}
