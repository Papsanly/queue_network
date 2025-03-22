use crate::{
    stats::{Stats, StepStats},
    weighted_average::weighted_average,
};
use std::{fmt::Debug, time::Duration};

pub struct Devices {
    pub count: usize,
    pub idle: usize,
    pub workloads: Vec<(Duration, f32)>,
}

#[derive(Debug)]
pub struct DevicesStats {
    final_workload: f32,
    average_workload: f32,
}

#[derive(Debug)]
pub struct DevicesStepStats {
    workload: f32,
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

impl Stats for Devices {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(DevicesStats {
            final_workload: self.workload(),
            average_workload: weighted_average(&self.workloads),
        })
    }
}

impl StepStats for Devices {
    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(DevicesStepStats {
            workload: self.workload(),
        })
    }
}
