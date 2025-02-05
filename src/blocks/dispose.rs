use crate::{
    any::AsAny,
    blocks::{Block, BlockId},
    events::Event,
};
use std::{collections::BinaryHeap, time::Instant};

pub struct DisposeBlock {
    pub id: BlockId,
    pub disposed_events: usize,
}

impl DisposeBlock {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            disposed_events: 0,
        }
    }
}

impl AsAny for DisposeBlock {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Block for DisposeBlock {
    fn id(&self) -> BlockId {
        self.id
    }

    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {
        self.disposed_events += 1;
    }

    fn process_out(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}
}
