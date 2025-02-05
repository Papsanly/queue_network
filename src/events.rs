use std::{cmp::Ordering, time::Instant};

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    In,
    Out,
}

pub struct Event<Block>(pub Instant, pub Block, pub EventType);

impl<Block> PartialEq<Event<Block>> for Event<Block> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Block> Eq for Event<Block> {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op because we know that `Instant` is `Eq`
    }
}

impl<Block> PartialOrd<Event<Block>> for Event<Block> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Block> Ord for Event<Block> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}
