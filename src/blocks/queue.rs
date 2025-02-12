use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Queue {
    pub length: usize,
    pub capacity: Option<usize>,
    lengths: Vec<(Instant, usize)>,
}

impl Queue {
    pub fn from_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity: Some(capacity),
            lengths: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, current_time: Instant) {
        self.length += 1;
        self.lengths.push((current_time, self.length));
    }

    pub fn dequeue(&mut self, current_time: Instant) {
        self.length -= 1;
        self.lengths.push((current_time, self.length));
    }

    pub fn total_weighted_time(&self) -> f32 {
        let mut total = 0.0;
        let mut iter = self.lengths.iter();
        let Some((mut current_time, _)) = iter.next() else {
            return 0.0;
        };
        for &(time, length) in iter {
            total += (time - current_time).as_secs_f32() * length as f32;
            current_time = time;
        }
        total
    }

    pub fn duration(&self) -> Duration {
        let Some(&first) = self.lengths.first().map(|(time, _)| time) else {
            return Duration::from_secs(0);
        };
        let Some(&last) = self.lengths.last().map(|(time, _)| time) else {
            return Duration::from_secs(0);
        };
        last - first
    }

    pub fn average_length(&self) -> f32 {
        self.total_weighted_time() / self.duration().as_secs_f32()
    }
}
