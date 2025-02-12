use crate::{
    blocks::{Block, BlockId, BlockType},
    events::{Event, EventType},
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::Duration,
};

pub struct QueueNetwork {
    event_queue: BinaryHeap<Event>,
    real_time: bool,
    #[allow(clippy::type_complexity)]
    on_simulation_step: Box<dyn Fn(&QueueNetwork, Event)>,
    pub blocks: HashMap<BlockId, BlockType>,
}

impl QueueNetwork {
    pub fn new() -> QueueNetwork {
        QueueNetwork {
            event_queue: BinaryHeap::new(),
            real_time: false,
            on_simulation_step: Box::new(|_, _| {}),
            blocks: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn real_time(mut self) -> Self {
        self.real_time = true;
        self
    }

    pub fn add_block(mut self, block: impl Into<BlockType>) -> Self {
        let block = block.into();
        self.blocks.insert(block.id(), block);
        self
    }

    #[allow(unused)]
    pub fn on_simulation_step(
        mut self,
        on_simulation_step: impl Fn(&QueueNetwork, Event) + 'static,
    ) -> Self {
        self.on_simulation_step = Box::new(on_simulation_step);
        self
    }

    pub fn simulate(&mut self, duration: Duration) {
        for block in self.blocks.values_mut() {
            block.init(&mut self.event_queue);
        }

        while let Some(Event(time, block_id, event_type)) = self.event_queue.pop() {
            if self.real_time {
                std::thread::sleep(time);
            }
            if time >= duration {
                break;
            }
            let block = self
                .blocks
                .get_mut(&block_id)
                .expect("event queue should only contain valid block ids");
            match event_type {
                EventType::In => block.process_in(&mut self.event_queue, time),
                EventType::Out => {
                    block.process_out(&mut self.event_queue, time);
                    if let Some(next) = block.next() {
                        self.event_queue.push(Event(time, next, EventType::In));
                    }
                }
            }
            (self.on_simulation_step)(&self, Event(time, block_id, event_type));
        }
    }
}
