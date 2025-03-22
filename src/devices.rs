use std::time::Duration;

pub struct Devices {
    pub count: usize,
    pub idle: usize,
    pub workloads: Vec<(Duration, f32)>,
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

    pub fn load(&mut self, simulation_duration: Duration) {
        self.idle = self.idle.checked_sub(1).expect("all devices are busy");
        self.workloads.push((simulation_duration, self.workload()));
    }

    pub fn unload(&mut self, simulation_duration: Duration) {
        if self.count != self.idle {
            self.idle += 1;
            self.workloads.push((simulation_duration, self.workload()));
        }
    }

    pub fn workload(&self) -> f32 {
        (self.count - self.idle) as f32 / self.count as f32
    }
}
