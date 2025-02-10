use crate::{
    blocks::{queue::Queue, Block, BlockId, DistributionType},
    events::{Event, EventType},
    routers::{Router, RouterType},
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
    router: RouterType,
    distribution: DistributionType,
}

pub struct ProcessBlockBuilder<Distribution, Router> {
    id: BlockId,
    router: Router,
    distribution: Distribution,
    max_queue_length: Option<usize>,
}

impl<Distribution> ProcessBlockBuilder<Distribution, ()> {
    pub fn router(
        self,
        router: impl Into<RouterType>,
    ) -> ProcessBlockBuilder<Distribution, RouterType> {
        ProcessBlockBuilder {
            id: self.id,
            max_queue_length: self.max_queue_length,
            distribution: self.distribution,
            router: router.into(),
        }
    }
}

impl<Router> ProcessBlockBuilder<(), Router> {
    pub fn distribution(
        self,
        distribution: impl Into<DistributionType>,
    ) -> ProcessBlockBuilder<DistributionType, Router> {
        ProcessBlockBuilder {
            id: self.id,
            router: self.router,
            max_queue_length: self.max_queue_length,
            distribution: distribution.into(),
        }
    }
}

impl<Distribution, Router> ProcessBlockBuilder<Distribution, Router> {
    pub fn max_queue_length(mut self, max_queue_length: usize) -> Self {
        self.max_queue_length = Some(max_queue_length);
        self
    }
}

impl ProcessBlockBuilder<DistributionType, RouterType> {
    pub fn build(self) -> ProcessBlock {
        ProcessBlock {
            id: self.id,
            queue: self
                .max_queue_length
                .map(Queue::from_capacity)
                .unwrap_or_default(),
            router: self.router,
            distribution: self.distribution,
        }
    }
}

impl ProcessBlock {
    pub fn builder(id: BlockId) -> ProcessBlockBuilder<(), ()> {
        ProcessBlockBuilder {
            id,
            router: (),
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

    fn next(&self) -> Option<BlockId> {
        self.router.next()
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
