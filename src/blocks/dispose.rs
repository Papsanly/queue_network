use super::Block;
use crate::{BlockId, Event};
use std::{any::Any, collections::BinaryHeap, time::Instant};

#[derive(Default)]
pub struct DisposeBlock {
    pub disposed_events: usize,
}

impl Block<BlockId> for DisposeBlock {
    fn init(&mut self, _event_queue: &mut BinaryHeap<Event<BlockId>>, _current_time: Instant) {}

    fn process_in(
        &mut self,
        _event_queue: &mut BinaryHeap<Event<BlockId>>,
        _current_time: Instant,
    ) {
        self.disposed_events += 1;
    }

    fn process_out(
        &mut self,
        _event_queue: &mut BinaryHeap<Event<BlockId>>,
        _current_time: Instant,
    ) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
