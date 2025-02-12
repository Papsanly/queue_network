use crate::blocks::BlockId;
use std::{cmp::Ordering, time::Duration};

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    In,
    Out,
}

pub struct Event(pub Duration, pub BlockId, pub EventType);

impl PartialEq<Event> for Event {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Event {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op because we know that `Instant` is `Eq`
    }
}

impl PartialOrd<Event> for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}
