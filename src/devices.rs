use crate::{
    stats::{Stats, StepStats},
    weighted_average::weighted_average,
};
use std::{fmt::Debug, time::Duration};

pub struct Devices {
    pub busy: usize,
    pub workers: Vec<Option<usize>>,
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
            busy: 0,
            workers: vec![None; count],
            workloads: Vec::new(),
        }
    }

    pub fn idle(&self) -> usize {
        self.workers.len() - self.busy
    }

    pub fn count(&self) -> usize {
        self.workers.len()
    }

    pub fn load(&mut self, event_id: usize, simulation_duration: Duration) {
        if self.workers.len() == self.busy {
            panic!("all devices are busy");
        }
        let available_worker_idx = self
            .workers
            .iter()
            .position(|i| i.is_none())
            .expect("some worker should be available");
        self.workers[available_worker_idx] = Some(event_id);
        self.busy += 1;
        self.workloads.push((simulation_duration, self.workload()));
    }

    pub fn unload(&mut self, event_id: usize, simulation_duration: Duration) {
        if self.busy == 0 {
            panic!("no devices are busy");
        }
        let event_idx = self
            .workers
            .iter()
            .position(|i| i.is_some_and(|e| e == event_id))
            .expect("event id to unload should be in the list of workers");
        self.workers[event_idx] = None;
        self.busy -= 1;
        self.workloads.push((simulation_duration, self.workload()));
    }

    pub fn workload(&self) -> f32 {
        self.busy as f32 / self.workers.len() as f32
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
