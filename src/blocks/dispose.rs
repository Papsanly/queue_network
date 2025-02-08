use crate::{
    blocks::{BlockId, BlockTrait, Stats},
    events::Event,
};
use std::{collections::BinaryHeap, time::Instant};

#[allow(unused)]
#[derive(Debug)]
pub struct DisposeBlockStats {
    pub disposed_events: usize,
}

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

impl Stats<DisposeBlockStats> for DisposeBlock {
    fn stats(&self) -> DisposeBlockStats {
        DisposeBlockStats {
            disposed_events: self.disposed_events,
        }
    }
}

impl BlockTrait for DisposeBlock {
    fn id(&self) -> BlockId {
        self.id
    }

    fn links(&self) -> &[BlockId] {
        &[]
    }

    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {
        self.disposed_events += 1;
    }

    fn process_out(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}
}
