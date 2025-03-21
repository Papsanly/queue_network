use crate::{
    stats::{Stats, StepStats},
    weighted_average::weighted_average,
};
use std::{fmt::Debug, time::Duration};

#[derive(Default)]
pub struct Queue {
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

impl Queue {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(capacity),
            lengths: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, simulation_duration: Duration) {
        self.length += 1;
        self.lengths.push((simulation_duration, self.length));
    }

    pub fn dequeue(&mut self, simulation_duration: Duration) {
        self.length -= 1;
        self.lengths.push((simulation_duration, self.length));
    }
}

impl Stats for Queue {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStats {
            final_length: self.length,
            average_length: weighted_average(&self.lengths),
        })
    }
}

impl StepStats for Queue {
    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(QueueStepStats {
            length: self.length,
        })
    }
}
