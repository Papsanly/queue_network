use super::Block;
use crate::{events::EventType, BlockId, Event};
use rand::rng;
use rand_distr::{Distribution, Normal};
use std::{
    any::Any,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct ProcessBlock {
    pub queue_length: usize,
    pub max_queue_length: Option<usize>,
    pub queue_lengths: Vec<(Instant, usize)>,
    pub rejections: usize,
}

impl ProcessBlock {
    fn delay(&self) -> Duration {
        Duration::from_secs_f64(Normal::new(8.0, 0.0).unwrap().sample(&mut rng()))
    }
}

impl Block<BlockId> for ProcessBlock {
    fn init(&mut self, _event_queue: &mut BinaryHeap<Event<BlockId>>, _current_time: Instant) {}

    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant) {
        let max_queue_length = self.max_queue_length.unwrap_or(usize::MAX);
        match self.queue_length {
            0 => {
                event_queue.push(Event(
                    current_time + self.delay(),
                    BlockId::Process,
                    EventType::Out,
                ));
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

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant) {
        event_queue.push(Event(current_time, BlockId::Dispose, EventType::In));
        if self.queue_length > 0 {
            self.queue_length -= 1;
            self.queue_lengths.push((current_time, self.queue_length));
            event_queue.push(Event(
                current_time + self.delay(),
                BlockId::Process,
                EventType::Out,
            ));
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
