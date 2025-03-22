use crate::{
    blocks::{Block, BlockId},
    events::{Event, EventType},
    routers::Router,
    stats::{Stats, StepStats},
};
use rand::{distr::Distribution, rng, Rng};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    time::Duration,
};

pub struct CreateBlockBuilder<Distribution, Router> {
    id: BlockId,
    first_at: Duration,
    router: Router,
    distribution: Distribution,
}

impl<R> CreateBlockBuilder<(), R> {
    pub fn distribution<D: Distribution<f32>>(self, distribution: D) -> CreateBlockBuilder<D, R> {
        CreateBlockBuilder {
            id: self.id,
            first_at: self.first_at,
            router: self.router,
            distribution,
        }
    }
}

impl<D> CreateBlockBuilder<D, ()> {
    pub fn router<R: Router>(self, router: R) -> CreateBlockBuilder<D, R> {
        CreateBlockBuilder {
            id: self.id,
            first_at: self.first_at,
            distribution: self.distribution,
            router,
        }
    }
}

impl<D, R> CreateBlockBuilder<D, R> {
    pub fn first_at(mut self, first_at: Duration) -> CreateBlockBuilder<D, R> {
        self.first_at = first_at;
        self
    }
}

impl<D: Distribution<f32>, R: Router> CreateBlockBuilder<D, R> {
    pub fn build(self) -> CreateBlock<D, R> {
        CreateBlock {
            id: self.id,
            first_at: self.first_at,
            created_events: 0,
            router: self.router,
            distribution: self.distribution,
        }
    }
}

#[derive(Debug)]
pub struct CreateBlockStats {
    pub created_events: usize,
}

pub struct CreateBlock<D, R> {
    pub id: BlockId,
    pub created_events: usize,
    router: R,
    first_at: Duration,
    distribution: D,
}

impl CreateBlock<(), ()> {
    pub fn builder(id: BlockId) -> CreateBlockBuilder<(), ()> {
        CreateBlockBuilder {
            id,
            first_at: Duration::ZERO,
            router: (),
            distribution: (),
        }
    }
}

impl<D: Distribution<f32>, R: Router> CreateBlock<D, R> {
    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl<D: Distribution<f32>, R: Router> Stats for CreateBlock<D, R> {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(CreateBlockStats {
            created_events: self.created_events,
        })
    }
}

impl<D: Distribution<f32>, R: Router> StepStats for CreateBlock<D, R> {
    fn step_stats(&self) -> Box<dyn Debug> {
        self.stats()
    }
}

impl<D: Distribution<f32>, R: Router> Block for CreateBlock<D, R> {
    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        self.router.next(blocks)
    }

    fn init(&mut self, event_queue: &mut BinaryHeap<Event>) {
        event_queue.push(Event(self.first_at, self.id, EventType::Out));
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, simulation_duration: Duration) {
        event_queue.push(Event(
            simulation_duration + self.delay(),
            self.id,
            EventType::Out,
        ));
        self.created_events += 1;
    }
}
