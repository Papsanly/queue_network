use crate::{
    blocks::{queue::Queue, Block, BlockId, Distribution},
    events::{Event, EventType},
};
use rand::{rng, Rng};
use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[allow(unused)]
#[derive(Debug)]
pub struct ProcessBlockStats {
    pub processed: usize,
    pub rejections: usize,
    pub rejection_probability: f32,
    pub average_queue_length: f32,
    pub average_waited_time: f32,
}

pub struct ProcessBlock {
    pub id: BlockId,
    pub queue: Queue,
    links: Vec<BlockId>,
    distribution: Distribution,
}

pub struct ProcessBlockBuilder<Distribution> {
    id: BlockId,
    links: Vec<BlockId>,
    distribution: Distribution,
    max_queue_length: Option<usize>,
}

impl ProcessBlockBuilder<()> {
    pub fn distribution(
        self,
        distribution: impl Into<Distribution>,
    ) -> ProcessBlockBuilder<Distribution> {
        ProcessBlockBuilder {
            id: self.id,
            links: self.links,
            max_queue_length: self.max_queue_length,
            distribution: distribution.into(),
        }
    }
}

impl<Distribution> ProcessBlockBuilder<Distribution> {
    pub fn add_link(mut self, block_id: BlockId) -> Self {
        self.links.push(block_id);
        self
    }

    pub fn max_queue_length(mut self, max_queue_length: usize) -> Self {
        self.max_queue_length = Some(max_queue_length);
        self
    }
}

impl ProcessBlockBuilder<Distribution> {
    pub fn build(self) -> ProcessBlock {
        ProcessBlock {
            id: self.id,
            queue: self
                .max_queue_length
                .map(Queue::from_capacity)
                .unwrap_or_default(),
            links: self.links,
            distribution: self.distribution,
        }
    }
}

impl ProcessBlock {
    pub fn builder(id: BlockId) -> ProcessBlockBuilder<()> {
        ProcessBlockBuilder {
            id,
            links: Vec::new(),
            distribution: (),
            max_queue_length: None,
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl Block for ProcessBlock {
    type Stats = ProcessBlockStats;

    fn id(&self) -> BlockId {
        self.id
    }

    fn links(&self) -> &[BlockId] {
        &self.links
    }

    fn stats(&self) -> ProcessBlockStats {
        ProcessBlockStats {
            processed: self.queue.processed,
            rejections: self.queue.rejections,
            rejection_probability: self.queue.rejections as f32
                / (self.queue.rejections + self.queue.processed) as f32,
            average_queue_length: self.queue.average_length(),
            average_waited_time: self.queue.average_waited_time(),
        }
    }

    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        match self.queue.length {
            0 => {
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
                self.queue.enqueue(current_time);
            }
            x if x < self.queue.capacity.unwrap_or(usize::MAX) => {
                self.queue.enqueue(current_time);
            }
            _ => {
                self.queue.rejections += 1;
            }
        }
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        match self.queue.length {
            0 => {}
            1 => {
                self.queue.dequeue(current_time);
            }
            _ => {
                self.queue.dequeue(current_time);
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
            }
        }
    }
}
