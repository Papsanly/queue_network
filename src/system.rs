use crate::{
    blocks::{Block, BlockId, BlockTrait},
    events::{Event, EventType},
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

pub struct QueueNetwork {
    event_queue: BinaryHeap<Event>,
    real_time: bool,
    pub blocks: HashMap<BlockId, Block>,
}

impl QueueNetwork {
    pub fn new() -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            real_time: false,
            blocks: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn real_time(mut self) -> Self {
        self.real_time = true;
        self
    }

    pub fn add_block(mut self, block: impl Into<Block>) -> Self {
        let block = block.into();
        self.blocks.insert(block.id(), block);
        self
    }

    pub fn simulate(
        &mut self,
        duration: Duration,
        on_simulation_step: impl Fn(Instant, &Block, EventType),
    ) {
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
            on_simulation_step(current_time, block, event_type);
        }
    }
}
