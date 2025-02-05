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

#[derive(Default)]
pub struct Queue {
    pub length: usize,
    pub capacity: Option<usize>,
    lengths: Vec<(Instant, usize)>,
}

impl Queue {
    fn from_max_length(max_length: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(max_length),
            lengths: Vec::new(),
        }
    }

    fn enqueue(&mut self, current_time: Instant) {
        self.length += 1;
        self.lengths.push((current_time, self.length));
    }

    fn dequeue(&mut self, current_time: Instant) {
        self.length -= 1;
        self.lengths.push((current_time, self.length));
    }

    pub fn average_length(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.lengths.len() - 1 {
            let (start_time, start_length) = self.lengths[i];
            let (end_time, end_length) = self.lengths[i + 1];
            total +=
                (end_time - start_time).as_secs_f64() * (start_length + end_length) as f64 / 2.0;
        }
        total / (self.lengths.last().unwrap().0 - self.lengths.first().unwrap().0).as_secs_f64()
    }
}

pub struct ProcessBlock<D: Distribution<f64>> {
    pub id: BlockId,
    pub queue: Queue,
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
            queue: self
                .max_queue_length
                .map(Queue::from_max_length)
                .unwrap_or_default(),
            links: self.links,
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
        match self.queue.length {
            0 => {
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
                self.queue.enqueue(current_time);
            }
            x if x < self.queue.capacity.unwrap_or(usize::MAX) => {
                self.queue.enqueue(current_time);
            }
            _ => {
                self.rejections += 1;
            }
        }
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
        match self.queue.length {
            0 => {}
            1 => self.queue.dequeue(current_time),
            _ => {
                self.queue.dequeue(current_time);
                event_queue.push(Event(current_time + self.delay(), self.id, EventType::Out));
            }
        }
    }
}
