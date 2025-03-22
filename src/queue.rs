use std::time::Duration;

#[derive(Default)]
pub struct Queue {
    pub length: usize,
    pub capacity: Option<usize>,
    lengths: Vec<(Duration, usize)>,
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

    pub fn total_weighted_time(&self) -> f32 {
        let mut total = 0.0;
        let mut iter = self.lengths.iter();
        let Some((mut current_time, mut length)) = iter.next() else {
            return 0.0;
        };
        for &(time, new_length) in iter {
            total += (time - current_time).as_secs_f32() * length as f32;
            current_time = time;
            length = new_length;
        }
        total
    }

    pub fn duration(&self) -> Duration {
        let Some(&last) = self.lengths.last().map(|(time, _)| time) else {
            return Duration::from_secs(0);
        };
        last
    }

    pub fn average_length(&self) -> f32 {
        self.total_weighted_time() / self.duration().as_secs_f32()
    }
}
