use crate::{
    blocks::{Block, BlockId},
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

impl Block for DisposeBlock {
    type StepStats = DisposeBlockStats;
    type Stats = DisposeBlockStats;

    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self) -> Option<BlockId> {
        None
    }

    fn step_stats(&self) -> Self::StepStats {
        self.stats()
    }

    fn stats(&self) -> DisposeBlockStats {
        DisposeBlockStats {
            disposed_events: self.disposed_events,
        }
    }

    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {
        self.disposed_events += 1;
    }
}
