use std::time::{Duration, Instant};

pub struct Devices {
    pub count: usize,
    pub idle: usize,
    pub workloads: Vec<(Instant, f32)>,
}

impl Default for Devices {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Devices {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            idle: count,
            workloads: Vec::new(),
        }
    }

    pub fn load(&mut self, current_time: Instant) {
        self.idle = self.idle.checked_sub(1).expect("all devices are busy");
        self.workloads.push((current_time, self.workload()));
    }

    pub fn unload(&mut self, current_time: Instant) {
        if self.count != self.idle {
            self.idle += 1;
            self.workloads.push((current_time, self.workload()));
        }
    }

    pub fn workload(&self) -> f32 {
        (self.count - self.idle) as f32 / self.count as f32
    }

    pub fn total_weighted_time(&self) -> f32 {
        let mut total = 0.0;
        let mut iter = self.workloads.iter();
        let Some((mut current_time, _)) = iter.next() else {
            return 0.0;
        };
        for &(time, workload) in iter {
            total += (time - current_time).as_secs_f32() * workload;
            current_time = time;
        }
        total
    }

    pub fn duration(&self) -> Duration {
        let Some(&first) = self.workloads.first().map(|(time, _)| time) else {
            return Duration::from_secs(0);
        };
        let Some(&last) = self.workloads.last().map(|(time, _)| time) else {
            return Duration::from_secs(0);
        };
        last - first
    }

    pub fn average_workload(&self) -> f32 {
        self.total_weighted_time() / self.duration().as_secs_f32()
    }
}
