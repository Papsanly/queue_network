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

#[derive(Debug, Copy, Clone)]
pub enum ProcessBlockState {
    Idle,
    Busy,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ProcessBlockStats {
    pub processed: usize,
    pub rejections: usize,
    pub state: ProcessBlockState,
    pub rejection_probability: f32,
    pub average_queue_length: Option<f32>,
    pub average_waited_time: Option<f32>,
}

pub struct ProcessBlock {
    pub id: BlockId,
    pub queue: Option<Queue>,
    pub state: ProcessBlockState,
    pub processed: usize,
    pub rejections: usize,
    router: RouterType,
    distribution: DistributionType,
}

pub struct ProcessBlockBuilder<Distribution, Router> {
    id: BlockId,
    router: Router,
    distribution: Distribution,
    queue: Option<Queue>,
}

impl<Distribution> ProcessBlockBuilder<Distribution, ()> {
    pub fn router(
        self,
        router: impl Into<RouterType>,
    ) -> ProcessBlockBuilder<Distribution, RouterType> {
        ProcessBlockBuilder {
            id: self.id,
            queue: self.queue,
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
            queue: self.queue,
            distribution: distribution.into(),
        }
    }
}

impl<Distribution, Router> ProcessBlockBuilder<Distribution, Router> {
    pub fn queue(mut self, queue: Queue) -> Self {
        self.queue = Some(queue);
        self
    }
}

impl ProcessBlockBuilder<DistributionType, RouterType> {
    pub fn build(self) -> ProcessBlock {
        ProcessBlock {
            id: self.id,
            queue: self.queue,
            processed: 0,
            rejections: 0,
            state: ProcessBlockState::Idle,
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
            queue: None,
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
            processed: self.processed,
            rejections: self.rejections,
            rejection_probability: self.rejections as f32
                / (self.rejections + self.processed) as f32,
            state: self.state,
            average_queue_length: self.queue.as_ref().map(|q| q.average_length()),
            average_waited_time: self
                .queue
                .as_ref()
                .map(|q| q.total_waited_time() / self.processed as f32),
        }
    }

    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        match self.state {
            ProcessBlockState::Idle => {
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
                self.state = ProcessBlockState::Busy;
            }
            ProcessBlockState::Busy => {
                let Some(queue) = &mut self.queue else {
                    self.rejections += 1;
                    return;
                };
                if queue.length < queue.capacity.unwrap_or(usize::MAX) {
                    queue.enqueue(current_time);
                } else {
                    self.rejections += 1;
                }
            }
        }
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        self.processed += 1;
        let delay = self.delay();
        let Some(queue) = &mut self.queue else {
            self.state = ProcessBlockState::Idle;
            return;
        };
        match queue.length {
            0 => self.state = ProcessBlockState::Idle,
            1 => queue.dequeue(current_time),
            _ => {
                queue.dequeue(current_time);
                event_queue.push(Event(current_time + delay, self.id, EventType::Out));
            }
        }
    }
}
