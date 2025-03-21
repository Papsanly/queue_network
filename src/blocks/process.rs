use crate::{
    blocks::{Block, BlockId},
    devices::Devices,
    events::{Event, EventType},
    queue::Queue,
    routers::Router,
};
use rand::{distr::Distribution, rng, Rng};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    time::Duration,
};

#[derive(Debug)]
pub struct ProcessBlockStepStats {
    pub processed: usize,
    pub rejections: usize,
    pub workload: f32,
    pub rejection_probability: f32,
    pub queue_length: usize,
}

#[derive(Debug)]
pub struct ProcessBlockStats {
    pub processed: usize,
    pub rejections: usize,
    pub final_workload: f32,
    pub average_workload: f32,
    pub rejection_probability: f32,
    pub final_queue_length: usize,
    pub average_queue_length: f32,
    pub average_waited_time: f32,
}

pub struct ProcessBlock<D, R> {
    pub id: BlockId,
    pub queue: Option<Queue>,
    pub devices: Devices,
    pub processed: usize,
    pub rejections: usize,
    router: R,
    distribution: D,
}

pub struct ProcessBlockBuilder<Distribution, Router> {
    id: BlockId,
    router: Router,
    distribution: Distribution,
    devices: Devices,
    queue: Option<Queue>,
}

impl<D> ProcessBlockBuilder<D, ()> {
    pub fn router<R: Router>(self, router: R) -> ProcessBlockBuilder<D, R> {
        ProcessBlockBuilder {
            id: self.id,
            queue: self.queue,
            devices: self.devices,
            distribution: self.distribution,
            router,
        }
    }
}

impl<R> ProcessBlockBuilder<(), R> {
    pub fn distribution<D: Distribution<f32>>(self, distribution: D) -> ProcessBlockBuilder<D, R> {
        ProcessBlockBuilder {
            id: self.id,
            router: self.router,
            queue: self.queue,
            devices: self.devices,
            distribution,
        }
    }
}

impl<Distribution, Router> ProcessBlockBuilder<Distribution, Router> {
    pub fn queue(mut self, queue: impl Into<Queue>) -> Self {
        self.queue = Some(queue.into());
        self
    }

    pub fn devices(mut self, devices: impl Into<Devices>) -> Self {
        self.devices = devices.into();
        self
    }
}

impl<D: Distribution<f32>, R: Router> ProcessBlockBuilder<D, R> {
    pub fn build(self) -> ProcessBlock<D, R> {
        ProcessBlock {
            id: self.id,
            queue: self.queue,
            processed: 0,
            rejections: 0,
            devices: self.devices,
            router: self.router,
            distribution: self.distribution,
        }
    }
}

impl ProcessBlock<(), ()> {
    pub fn builder(id: BlockId) -> ProcessBlockBuilder<(), ()> {
        ProcessBlockBuilder {
            id,
            router: (),
            distribution: (),
            devices: Devices::default(),
            queue: None,
        }
    }
}

impl<D: Distribution<f32>, R: Router> ProcessBlock<D, R> {
    fn delay(&self) -> Duration {
        Duration::from_secs_f32(rng().sample(&self.distribution))
    }
}

impl<D: Distribution<f32>, R: Router> Block for ProcessBlock<D, R> {
    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        self.router.next(blocks)
    }

    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(ProcessBlockStepStats {
            processed: self.processed,
            rejections: self.rejections,
            workload: self.devices.workload(),
            rejection_probability: self.rejections as f32
                / (self.rejections + self.processed) as f32,
            queue_length: self.queue.as_ref().map(|q| q.length).unwrap_or(0),
        })
    }

    fn stats(&self) -> Box<dyn Debug> {
        Box::new(ProcessBlockStats {
            processed: self.processed,
            rejections: self.rejections,
            final_workload: self.devices.workload(),
            average_workload: self.devices.average_workload(),
            rejection_probability: self.rejections as f32
                / (self.rejections + self.processed) as f32,
            final_queue_length: self.queue.as_ref().map(|q| q.length).unwrap_or(0),
            average_queue_length: self
                .queue
                .as_ref()
                .map(|q| q.average_length())
                .unwrap_or(0.0),
            average_waited_time: self
                .queue
                .as_ref()
                .map(|q| q.total_weighted_time() / self.processed as f32)
                .unwrap_or(0.0),
        })
    }

    fn queue(&self) -> Option<&Queue> {
        self.queue.as_ref()
    }

    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, simulation_duration: Duration) {
        if self.devices.idle != 0 {
            event_queue.push(Event(
                simulation_duration + self.delay(),
                self.id,
                EventType::Out,
            ));
            self.devices.load(simulation_duration);
        } else {
            let Some(queue) = &mut self.queue else {
                self.rejections += 1;
                return;
            };
            if queue.length < queue.capacity.unwrap_or(usize::MAX) {
                queue.enqueue(simulation_duration);
            } else {
                self.rejections += 1;
            }
        }
    }

    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, simulation_duration: Duration) {
        self.processed += 1;
        let delay = self.delay();
        let Some(queue) = &mut self.queue else {
            self.devices.unload(simulation_duration);
            return;
        };
        if queue.length == 0 {
            self.devices.unload(simulation_duration);
        } else {
            queue.dequeue(simulation_duration);
            event_queue.push(Event(simulation_duration + delay, self.id, EventType::Out));
        }
    }
}
