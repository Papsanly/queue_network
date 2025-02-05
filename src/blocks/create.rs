use crate::{
    any::AsAny,
    blocks::{Block, BlockId},
    events::{Event, EventType},
};
use rand::rng;
use rand_distr::Distribution;
use std::{
    any::Any,
    collections::BinaryHeap,
    marker::PhantomData,
    time::{Duration, Instant},
};

pub struct CreateBlock<D: Distribution<f64>> {
    pub id: BlockId,
    pub created_events: usize,
    pub links: Vec<BlockId>,
    distribution: D,
}

pub struct CreateBlockBuilder<State, D: Distribution<f64>> {
    _p: PhantomData<State>,
    id: BlockId,
    distribution: Option<D>,
    links: Vec<BlockId>,
}

pub struct WithDistribution;
pub struct WithoutDistribution;

impl<D: Distribution<f64>> CreateBlockBuilder<WithoutDistribution, D> {
    pub fn distribution(self, distribution: D) -> CreateBlockBuilder<WithDistribution, D> {
        CreateBlockBuilder {
            _p: PhantomData,
            id: self.id,
            distribution: Some(distribution),
            links: self.links,
        }
    }
}

impl<D: Distribution<f64>> CreateBlockBuilder<WithDistribution, D> {
    pub fn add_link(mut self, block_id: BlockId) -> Self {
        self.links.push(block_id);
        self
    }

    pub fn build(self) -> CreateBlock<D> {
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

impl<D: Distribution<f64>> CreateBlock<D> {
    pub fn builder(id: BlockId) -> CreateBlockBuilder<WithoutDistribution, D> {
        CreateBlockBuilder {
            _p: PhantomData,
            id,
            distribution: None,
            links: Vec::new(),
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f64(self.distribution.sample(&mut rng()))
    }
}

impl<D: Distribution<f64> + 'static> AsAny for CreateBlock<D> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<D: Distribution<f64> + 'static> Block for CreateBlock<D> {
    fn id(&self) -> BlockId {
        self.id
    }

    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        event_queue.push(Event(current_time, self.id, EventType::Out));
    }

    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        for link in &self.links {
            event_queue.push(Event(current_time, link, EventType::In));
        }
        event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
        self.created_events += 1;
    }
}
