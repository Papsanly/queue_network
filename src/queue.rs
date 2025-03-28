use crate::{
    stats::{Stats, StepStats},
    weighted_average::weighted_average,
};
use std::{collections::VecDeque, fmt::Debug, time::Duration};

#[derive(Default)]
pub struct Queue {
    queue: VecDeque<usize>,
    pub capacity: Option<usize>,
    pub lengths: Vec<(Duration, usize)>,
}

#[derive(Debug)]
pub struct QueueStats {
    final_length: usize,
    average_length: f32,
}

#[derive(Debug)]
pub struct QueueStepStats {
    length: usize,
}

impl Queue {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity: Some(capacity),
            lengths: Vec::new(),
        }
    }
}

impl Queue {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn enqueue(&mut self, event_id: usize, simulation_duration: Duration) {
        self.queue.push_back(event_id);
        self.lengths.push((simulation_duration, self.queue.len()));
    }

    pub fn dequeue(&mut self, simulation_duration: Duration) -> usize {
        let event_id = self
            .queue
            .pop_front()
            .expect("queue should not be empty when dequeueing an event");
        self.lengths.push((simulation_duration, self.queue.len()));
        event_id
    }
}

impl Stats for Queue {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStats {
            final_length: self.queue.len(),
            average_length: weighted_average(&self.lengths),
        })
    }
}

impl StepStats for Queue {
    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStepStats {
            length: self.queue.len(),
        })
    }
}
