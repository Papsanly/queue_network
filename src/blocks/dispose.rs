use crate::{
    blocks::{Block, BlockId},
    events::Event,
    stats::{Stats, StepStats},
};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    time::Duration,
};

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

impl Stats for DisposeBlock {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(DisposeBlockStats {
            disposed_events: self.disposed_events,
        })
    }
}

impl StepStats for DisposeBlock {
    fn step_stats(&self) -> Box<dyn Debug> {
        self.stats()
    }
}

impl Block for DisposeBlock {
    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self, _blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        None
    }

    fn process_in(
        &mut self,
        _event_id: usize,
        _event_queue: &mut BinaryHeap<Event>,
        _simulation_duration: Duration,
    ) {
        self.disposed_events += 1;
    }
}
