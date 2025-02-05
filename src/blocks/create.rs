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
pub struct CreateBlock {
    pub created_events: usize,
}

impl CreateBlock {
    fn delay(&self) -> Duration {
        Duration::from_secs_f64(Normal::new(5.0, 0.0).unwrap().sample(&mut rng()))
    }
}

impl Block<BlockId> for CreateBlock {
    fn init(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant) {
        event_queue.push(Event(current_time, BlockId::Create, EventType::Out));
    }

    fn process_in(
        &mut self,
        _event_queue: &mut BinaryHeap<Event<BlockId>>,
        _current_time: Instant,
    ) {
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant) {
        event_queue.push(Event(current_time, BlockId::Process, EventType::In));
        event_queue.push(Event(
            current_time + self.delay(),
            BlockId::Create,
            EventType::Out,
        ));
        self.created_events += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
