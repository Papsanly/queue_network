use crate::{
    queues::Queue,
    stats::{Stats, StepStats},
    weighted_average::{weighted_average, weighted_total},
};
use std::{fmt::Debug, time::Duration};

#[derive(Default)]
pub struct RegularQueue {
    pub length: usize,
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

impl RegularQueue {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(capacity),
            lengths: Vec::new(),
        }
    }
}

impl Queue for RegularQueue {
    fn length(&self) -> usize {
        self.length
    }

    fn weighted_total(&self) -> f32 {
        weighted_total(&self.lengths)
    }

    fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    fn enqueue(&mut self, simulation_duration: Duration) {
        self.length += 1;
        self.lengths.push((simulation_duration, self.length));
    }

    fn dequeue(&mut self, simulation_duration: Duration) {
        self.length -= 1;
        self.lengths.push((simulation_duration, self.length));
    }
}

impl Stats for RegularQueue {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStats {
            final_length: self.length,
            average_length: weighted_average(&self.lengths),
        })
    }
}

impl StepStats for RegularQueue {
    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStepStats {
            length: self.length,
        })
    }
}
