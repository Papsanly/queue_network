use crate::{
    blocks::{Block, BlockId},
    devices::Devices,
    events::{Event, EventType},
    queues::Queue,
    routers::Router,
    stats::{Stats, StepStats},
};
use rand::{distr::Distribution, rng, Rng};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    time::Duration,
};

#[derive(Debug)]
pub struct ProcessBlockStepStats<D, Q> {
    pub processed: usize,
    pub rejections: usize,
    pub devices: D,
    pub rejection_probability: f32,
    pub queue: Q,
}

#[derive(Debug)]
pub struct ProcessBlockStats<D, Q> {
    pub processed: usize,
    pub rejections: usize,
    pub devices: D,
    pub rejection_probability: f32,
    pub queue: Q,
    pub average_waited_time: f32,
}

pub struct ProcessBlock<D, R> {
    pub id: BlockId,
    pub queue: Option<Box<dyn Queue>>,
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
    queue: Option<Box<dyn Queue>>,
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
    pub fn queue(mut self, queue: impl Queue + 'static) -> Self {
        self.queue = Some(Box::new(queue));
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
        Duration::from_secs_f32(rng().sample(&self.distribution).max(0.0))
    }
}

impl<D: Distribution<f32>, R: Router> Stats for ProcessBlock<D, R> {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(ProcessBlockStats {
            processed: self.processed,
            rejections: self.rejections,
            devices: self.devices.stats(),
            rejection_probability: self.rejections as f32
                / (self.rejections + self.processed) as f32,
            queue: self
                .queue
                .as_ref()
                .map(|q| q.stats())
                .unwrap_or(Box::new(None::<()>)),
            average_waited_time: self
                .queue
                .as_ref()
                .map(|q| q.weighted_total() / self.processed as f32)
                .unwrap_or(0.0),
        })
    }
}

impl<D: Distribution<f32>, R: Router> StepStats for ProcessBlock<D, R> {
    fn step_stats(&self) -> Box<dyn Debug> {
        Box::new(ProcessBlockStepStats {
            processed: self.processed,
            rejections: self.rejections,
            devices: self.devices.step_stats(),
            rejection_probability: self.rejections as f32
                / (self.rejections + self.processed) as f32,
            queue: self
                .queue
                .as_ref()
                .map(|q| q.step_stats())
                .unwrap_or(Box::new(None::<()>)),
        })
    }
}

impl<D: Distribution<f32>, R: Router> Block for ProcessBlock<D, R> {
    fn id(&self) -> BlockId {
        self.id
    }

    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        self.router.next(blocks)
    }

    fn queue(&self) -> Option<&dyn Queue> {
        self.queue.as_deref()
    }

    fn init(&mut self, event_queue: &mut BinaryHeap<Event>) {
        for event_id in &self.devices.workers {
            if let &Some(event_id) = event_id {
                event_queue.push(Event(self.delay(), self.id, EventType::Out, event_id));
            }
        }
    }

    fn process_in(
        &mut self,
        event_id: usize,
        event_queue: &mut BinaryHeap<Event>,
        simulation_duration: Duration,
    ) {
        if self.devices.idle() != 0 {
            self.devices.load(event_id, simulation_duration);
            event_queue.push(Event(
                simulation_duration + self.delay(),
                self.id,
                EventType::Out,
                event_id,
            ));
        } else {
            let Some(queue) = &mut self.queue else {
                self.rejections += 1;
                return;
            };
            if queue.length() < queue.capacity().unwrap_or(usize::MAX) {
                queue.enqueue(event_id, simulation_duration);
            } else {
                self.rejections += 1;
            }
        }
    }

    fn process_out(
        &mut self,
        event_id: usize,
        event_queue: &mut BinaryHeap<Event>,
        simulation_duration: Duration,
    ) {
        self.processed += 1;
        let delay = self.delay();
        let Some(queue) = &mut self.queue else {
            self.devices.unload(event_id, simulation_duration);
            return;
        };
        self.devices.unload(event_id, simulation_duration);
        if queue.length() != 0 {
            let next_event_id = queue.dequeue(simulation_duration);
            self.devices.load(next_event_id, simulation_duration);
            event_queue.push(Event(
                simulation_duration + delay,
                self.id,
                EventType::Out,
                next_event_id,
            ));
        }
    }
}
