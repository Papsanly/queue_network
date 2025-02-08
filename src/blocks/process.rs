use crate::{
    blocks::{BlockId, BlockTrait, Distribution, Stats},
    events::{Event, EventType},
};
use rand::{rng, Rng};
use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Queue {
    pub length: usize,
    pub capacity: Option<usize>,
    pub processed: usize,
    lengths: Vec<(Instant, usize)>,
}

impl Queue {
    fn from_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(capacity),
            processed: 0,
            lengths: Vec::new(),
        }
    }

    fn enqueue(&mut self, current_time: Instant) {
        self.length += 1;
        self.lengths.push((current_time, self.length));
    }

    fn dequeue(&mut self, current_time: Instant) {
        self.length -= 1;
        self.processed += 1;
        self.lengths.push((current_time, self.length));
    }

    pub fn total_waited_time(&self) -> f32 {
        let mut total = 0.0;
        let mut iter = self.lengths.iter();
        let (mut current_time, _) = iter
            .next()
            .expect("queue lengths must contain at least one element");
        for &(time, length) in iter {
            total += (time - current_time).as_secs_f32() * length as f32;
            current_time = time;
        }
        total
    }

    pub fn average_waited_time(&self) -> f32 {
        self.total_waited_time() / self.processed as f32
    }

    fn duration(&self) -> Duration {
        self.lengths.last().unwrap().0 - self.lengths.first().unwrap().0
    }

    pub fn average_length(&self) -> f32 {
        self.total_waited_time() / self.duration().as_secs_f32()
    }
}

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
    pub rejections: usize,
    links: Vec<BlockId>,
    distribution: Distribution,
}

pub struct ProcessBlockBuilder<const WITH_DISTRIBUTION: bool> {
    id: BlockId,
    links: Vec<BlockId>,
    distribution: Option<Distribution>,
    max_queue_length: Option<usize>,
}

impl ProcessBlockBuilder<false> {
    pub fn distribution(self, distribution: impl Into<Distribution>) -> ProcessBlockBuilder<true> {
        ProcessBlockBuilder {
            id: self.id,
            links: self.links,
            max_queue_length: self.max_queue_length,
            distribution: Some(distribution.into()),
        }
    }
}

impl<const WITH_DISTRIBUTION: bool> ProcessBlockBuilder<WITH_DISTRIBUTION> {
    pub fn add_link(mut self, block_id: BlockId) -> Self {
        self.links.push(block_id);
        self
    }

    pub fn max_queue_length(mut self, max_queue_length: usize) -> Self {
        self.max_queue_length = Some(max_queue_length);
        self
    }
}

impl ProcessBlockBuilder<true> {
    pub fn build(self) -> ProcessBlock {
        ProcessBlock {
            id: self.id,
            queue: self
                .max_queue_length
                .map(Queue::from_capacity)
                .unwrap_or_default(),
            links: self.links,
            rejections: 0,
            distribution: self
                .distribution
                .expect("distribution is Some because builder state is WithDistribution"),
        }
    }
}

impl ProcessBlock {
    pub fn builder(id: BlockId) -> ProcessBlockBuilder<false> {
        ProcessBlockBuilder {
            id,
            links: Vec::new(),
            distribution: None,
            max_queue_length: None,
        }
    }

    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl Stats<ProcessBlockStats> for ProcessBlock {
    fn stats(&self) -> ProcessBlockStats {
        ProcessBlockStats {
            processed: self.queue.processed,
            rejections: self.rejections,
            rejection_probability: self.rejections as f32
                / (self.rejections + self.queue.processed) as f32,
            average_queue_length: self.queue.average_length(),
            average_waited_time: self.queue.average_waited_time(),
        }
    }
}

impl BlockTrait for ProcessBlock {
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
