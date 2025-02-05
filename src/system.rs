use crate::{
    blocks::{Block, BlockId},
    events::{Event, EventType},
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

pub struct DiscreteEventSystem {
    event_queue: BinaryHeap<Event>,
    real_time: bool,
    pub blocks: HashMap<BlockId, Box<dyn Block>>,
}

impl DiscreteEventSystem {
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

    pub fn add_block(mut self, block: impl Block + 'static) -> Self {
        self.blocks.insert(block.id(), Box::new(block));
        self
    }

    pub fn simulate(
        &mut self,
        duration: Duration,
        on_simulation_step: impl Fn(Instant, &dyn Block, EventType),
    ) {
        let start = Instant::now();
        let end = start + duration;
        let mut current_time = start;

        for block in self.blocks.values_mut() {
            block.init(&mut self.event_queue, current_time);
        }

        while current_time < end {
            let Event(time, block_id, event_type) = match self.event_queue.pop() {
                Some(time) => time,
                None => break,
            };
            if self.real_time {
                std::thread::sleep(time - current_time);
            }
            current_time = time;
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
            on_simulation_step(current_time, block.as_ref(), event_type);
        }
    }
}
