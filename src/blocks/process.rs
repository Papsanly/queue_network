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

pub struct ProcessBlock<D: Distribution<f64>> {
    pub id: BlockId,
    pub queue_length: usize,
    pub max_queue_length: Option<usize>,
    pub queue_lengths: Vec<(Instant, usize)>,
    pub rejections: usize,
    links: Vec<BlockId>,
    distribution: D,
}

pub struct ProcessBlockBuilder<State, D: Distribution<f64>> {
    _p: PhantomData<State>,
    id: BlockId,
    links: Vec<BlockId>,
    distribution: Option<D>,
    max_queue_length: Option<usize>,
}

pub struct WithDistribution;
pub struct WithoutDistribution;

impl<D: Distribution<f64>> ProcessBlockBuilder<WithoutDistribution, D> {
    pub fn distribution(self, distribution: D) -> ProcessBlockBuilder<WithDistribution, D> {
        ProcessBlockBuilder {
            _p: PhantomData,
            id: self.id,
            links: self.links,
            distribution: Some(distribution),
            max_queue_length: self.max_queue_length,
        }
    }
}

impl<State, D: Distribution<f64>> ProcessBlockBuilder<State, D> {
    pub fn add_link(mut self, block_id: BlockId) -> Self {
        self.links.push(block_id);
        self
    }

    pub fn max_queue_length(self, max_queue_length: usize) -> ProcessBlockBuilder<State, D> {
        ProcessBlockBuilder {
            _p: PhantomData,
            id: self.id,
            links: self.links,
            distribution: self.distribution,
            max_queue_length: Some(max_queue_length),
        }
    }
}

impl<D: Distribution<f64>> ProcessBlockBuilder<WithDistribution, D> {
    pub fn build(self) -> ProcessBlock<D> {
        ProcessBlock {
            id: self.id,
            queue_length: 0,
            links: self.links,
            max_queue_length: self.max_queue_length,
            queue_lengths: Vec::new(),
            rejections: 0,
            distribution: self
                .distribution
                .expect("distribution is Some because builder state is WithDistribution"),
        }
    }
}

impl<D: Distribution<f64>> ProcessBlock<D> {
    pub fn builder(id: BlockId) -> ProcessBlockBuilder<WithoutDistribution, D> {
        ProcessBlockBuilder {
            _p: PhantomData,
            id,
            links: Vec::new(),
            distribution: None,
            max_queue_length: None,
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f64(self.distribution.sample(&mut rng()))
    }
}

impl<D: Distribution<f64> + 'static> AsAny for ProcessBlock<D> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<D: Distribution<f64> + 'static> Block for ProcessBlock<D> {
    fn id(&self) -> BlockId {
        self.id
    }

    fn links(&self) -> &[BlockId] {
        &self.links
    }

    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>, _current_time: Instant) {}

    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        let max_queue_length = self.max_queue_length.unwrap_or(usize::MAX);
        match self.queue_length {
            0 => {
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
                self.queue_length += 1;
                self.queue_lengths.push((current_time, self.queue_length));
            }
            x if x < max_queue_length => {
                self.queue_length += 1;
                self.queue_lengths.push((current_time, self.queue_length));
            }
            _ => {
                self.rejections += 1;
            }
        }
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        if self.queue_length > 0 {
            self.queue_length -= 1;
            self.queue_lengths.push((current_time, self.queue_length));
            event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
        }
    }
}
