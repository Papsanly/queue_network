use crate::{
    blocks::{BlockId, BlockTrait, Distribution},
    events::{Event, EventType},
};
use rand::{rng, Rng};
use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

pub struct CreateBlockBuilder<const WITH_DISTRIBUTION: bool> {
    id: BlockId,
    links: Vec<BlockId>,
    distribution: Option<Distribution>,
}

impl CreateBlockBuilder<false> {
    pub fn distribution(self, distribution: impl Into<Distribution>) -> CreateBlockBuilder<true> {
        CreateBlockBuilder {
            id: self.id,
            links: self.links,
            distribution: Some(distribution.into()),
        }
    }
}

impl CreateBlockBuilder<true> {
    pub fn build(self) -> CreateBlock {
        CreateBlock {
            id: self.id,
            created_events: 0,
            links: self.links,
            distribution: self
                .distribution
                .expect("distribution is Some because builder state is WithDistribution"),
        }
    }
}

impl<const WITH_DISTRIBUTION: bool> CreateBlockBuilder<WITH_DISTRIBUTION> {
    pub fn add_link(mut self, link: BlockId) -> Self {
        self.links.push(link);
        self
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct CreateBlockStats {
    pub created_events: usize,
}

pub struct CreateBlock {
    pub id: BlockId,
    pub created_events: usize,
    pub links: Vec<BlockId>,
    distribution: Distribution,
}

impl CreateBlock {
    pub fn builder(id: BlockId) -> CreateBlockBuilder<false> {
        CreateBlockBuilder {
            id,
            links: Vec::new(),
            distribution: None,
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl BlockTrait for CreateBlock {
    type Stats = CreateBlockStats;

    fn id(&self) -> BlockId {
        self.id
    }

    fn links(&self) -> &[BlockId] {
        &self.links
    }

    fn stats(&self) -> CreateBlockStats {
        CreateBlockStats {
            created_events: self.created_events,
        }
    }

    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        event_queue.push(Event(current_time, self.id, EventType::Out));
    }

    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
        self.created_events += 1;
    }
}
