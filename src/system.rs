use crate::{
    blocks::{Block, BlockId, BlockType},
    events::{Event, EventType},
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

pub struct QueueNetwork<F: Fn(Duration, &BlockType, EventType)> {
    event_queue: BinaryHeap<Event>,
    real_time: bool,
    on_simulation_step: Option<F>,
    pub blocks: HashMap<BlockId, BlockType>,
}

impl<F: Fn(Duration, &BlockType, EventType)> QueueNetwork<F> {
    pub fn new() -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            real_time: false,
            on_simulation_step: None,
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

    pub fn on_simulation_step(mut self, on_simulation_step: F) -> Self {
        self.on_simulation_step = Some(on_simulation_step);
        self
    }

    pub fn simulate(&mut self, duration: Duration) {
        let start = Instant::now();
        let end = start + duration;
        let mut current_time = start;

        for block in self.blocks.values_mut() {
            block.init(&mut self.event_queue, current_time);
        }

        while let Some(Event(time, block_id, event_type)) = self.event_queue.pop() {
            if self.real_time {
                std::thread::sleep(time - current_time);
            }
            current_time = time;
            if current_time >= end {
                break;
            }
            let block = self
                .blocks
                .get_mut(&block_id)
                .expect("event queue should only contain valid block ids");
            match event_type {
                EventType::In => block.process_in(&mut self.event_queue, current_time),
                EventType::Out => {
                    block.process_out(&mut self.event_queue, current_time);
                    for link in block.links() {
                        self.event_queue
                            .push(Event(current_time, link, EventType::In));
                    }
                }
            }
            if let Some(ref f) = self.on_simulation_step {
                f(current_time - start, block, event_type)
            }
        }
    }
}
