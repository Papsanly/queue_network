use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Queue {
    pub length: usize,
    pub capacity: Option<usize>,
    pub processed: usize,
    pub rejections: usize,
    lengths: Vec<(Instant, usize)>,
}

impl Queue {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(capacity),
            processed: 0,
            rejections: 0,
            lengths: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, current_time: Instant) {
        self.length += 1;
        self.lengths.push((current_time, self.length));
    }

    pub fn dequeue(&mut self, current_time: Instant) {
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

    pub fn duration(&self) -> Duration {
        self.lengths.last().unwrap().0 - self.lengths.first().unwrap().0
    }

    pub fn average_length(&self) -> f32 {
        self.total_waited_time() / self.duration().as_secs_f32()
    }
}
