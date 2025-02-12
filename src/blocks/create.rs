use crate::{
    blocks::{Block, BlockId, DistributionType},
    events::{Event, EventType},
    routers::{Router, RouterType},
};
use rand::{rng, Rng};
use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

pub struct CreateBlockBuilder<Distribution, Router> {
    id: BlockId,
    router: Router,
    distribution: Distribution,
}

impl<Router> CreateBlockBuilder<(), Router> {
    pub fn distribution(
        self,
        distribution: impl Into<DistributionType>,
    ) -> CreateBlockBuilder<DistributionType, Router> {
        CreateBlockBuilder {
            id: self.id,
            router: self.router,
            distribution: distribution.into(),
        }
    }
}

impl<Distribution> CreateBlockBuilder<Distribution, ()> {
    pub fn router(
        self,
        router: impl Into<RouterType>,
    ) -> CreateBlockBuilder<Distribution, RouterType> {
        CreateBlockBuilder {
            id: self.id,
            distribution: self.distribution,
            router: router.into(),
        }
    }
}

impl CreateBlockBuilder<DistributionType, RouterType> {
    pub fn build(self) -> CreateBlock {
        CreateBlock {
            id: self.id,
            created_events: 0,
            router: self.router,
            distribution: self.distribution,
        }
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
    router: RouterType,
    distribution: DistributionType,
}

impl CreateBlock {
    pub fn builder(id: BlockId) -> CreateBlockBuilder<(), ()> {
        CreateBlockBuilder {
            id,
            router: (),
            distribution: (),
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl Block for CreateBlock {
    type StepStats = CreateBlockStats;
    type Stats = CreateBlockStats;

    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self) -> Option<BlockId> {
        self.router.next()
    }

    fn step_stats(&self) -> Self::StepStats {
        self.stats()
    }

    fn stats(&self) -> CreateBlockStats {
        CreateBlockStats {
            created_events: self.created_events,
        }
    }

    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        event_queue.push(Event(current_time, self.id, EventType::Out));
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
        self.created_events += 1;
    }
}
