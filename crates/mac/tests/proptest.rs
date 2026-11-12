//! Property tests untuk `sig6g-mac` (10.000 kasus per properti di CI).
//! Properti: eksklusivitas slot, fairness round-robin, HARQ batas,
//! determinisme jadwal.

use proptest::prelude::*;

use sig6g_mac::harq::{HarqProcess, HarqState};
use sig6g_mac::scheduler::MacScheduler;

proptest! {
    #[test]
    fn schedule_at_most_one_ue_per_slot(
        ues in proptest::collection::vec(any::<u16>(), 1..8),
        slots in 0u64..100,
    ) {
        let mut sched = MacScheduler::new();
        for &ue in &ues {
            let _ = sched.register_ue(ue);
        }
        let active: Vec<u16> = ues.as_slice().to_vec();
        for slot in 0..slots {
            let chosen = sched.schedule(usize::try_from(slot).unwrap(), &active);
            prop_assert!(chosen.map_or(true, |ue| active.contains(&ue)));
        }
    }

    #[test]
    fn round_robin_every_active_ue_chosen(
        ues in proptest::collection::vec(any::<u16>(), 2..6),
    ) {
        let mut sched = MacScheduler::new();
        let mut remaining: std::collections::HashSet<u16> = std::collections::HashSet::new();
        for &ue in &ues {
            let _ = sched.register_ue(ue);
            remaining.insert(ue);
        }
        let active: Vec<u16> = ues.as_slice().to_vec();
        let n = active.len();
        for slot in 0..(n * 3) {
            if let Some(ue) = sched.schedule(slot, &active) {
                remaining.remove(&ue);
            }
        }
        prop_assert!(remaining.is_empty(), "beberapa UE tidak pernah dipilih");
    }

    #[test]
    fn schedule_deterministic_same_input(
        ues in proptest::collection::vec(any::<u16>(), 2..6),
        slot in 0u64..1000,
    ) {
        let mut sched1 = MacScheduler::new();
        let mut sched2 = MacScheduler::new();
        for &ue in &ues {
            let _ = sched1.register_ue(ue);
            let _ = sched2.register_ue(ue);
        }
        let active: Vec<u16> = ues.as_slice().to_vec();
        let slot_usize = usize::try_from(slot).unwrap();
        let r1 = sched1.schedule(slot_usize, &active);
        let r2 = sched2.schedule(slot_usize, &active);
        prop_assert_eq!(r1, r2);
    }
}

#[test]
fn harq_exhaustion_after_max_plus_one_nack() {
    let mut h = HarqProcess::new();
    h.transmit(b"test").unwrap();
    for _ in 0..4 {
        h.on_nack();
    }
    assert_eq!(h.state(), HarqState::Exhausted);
}

#[test]
fn harq_transmit_then_ack_returns_idle() {
    let mut h = HarqProcess::new();
    h.transmit(b"test").unwrap();
    h.on_ack();
    assert_eq!(h.state(), HarqState::Idle);
}
