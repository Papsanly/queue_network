mod blocks;
mod events;

use crate::{blocks::Block, events::EventType};
use blocks::{CreateBlock, DisposeBlock, ProcessBlock};
use events::Event;
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    thread,
    time::{Duration, Instant},
};

struct DiscreteEventSystem<BlockId> {
    event_queue: BinaryHeap<Event<BlockId>>,
    blocks: HashMap<BlockId, Box<dyn Block<BlockId>>>,
}

impl<BlockId: Eq + Hash + 'static> DiscreteEventSystem<BlockId> {
    fn new() -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            blocks: HashMap::new(),
        }
    }

    fn add_block<T: Block<BlockId> + 'static>(mut self, block_id: BlockId, block: T) -> Self {
        self.blocks.insert(block_id, Box::new(block));
        self
    }

    fn simulate(
        &mut self,
        duration: Duration,
        on_simulation_step: impl Fn(BlockId, &dyn Block<BlockId>, EventType),
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
            thread::sleep(time - current_time);
            current_time = time;
            let Some(block) = self.blocks.get_mut(&block_id) else {
                continue;
            };
            match event_type {
                EventType::In => block.process_in(&mut self.event_queue, current_time),
                EventType::Out => block.process_out(&mut self.event_queue, current_time),
            }
            on_simulation_step(block_id, block.as_ref(), event_type);
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum BlockId {
    Create,
    Process,
    Dispose,
}

fn main() {
    let mut system = DiscreteEventSystem::new()
        .add_block(BlockId::Create, CreateBlock::default())
        .add_block(BlockId::Process, ProcessBlock::default())
        .add_block(BlockId::Dispose, DisposeBlock::default());

    system.simulate(Duration::from_secs(100), |block_id, block, event_type| {
        if let Some(block) = block.as_any().downcast_ref::<CreateBlock>() {
            println!(
                "Block: {:?} | Event: {:?} | Created Events: {:?}",
                block_id, event_type, block.created_events
            );
        } else if let Some(block) = block.as_any().downcast_ref::<ProcessBlock>() {
            println!(
                "Block: {:?} | Event: {:?} | Queue Length: {:?} | Rejections: {:?}",
                block_id, event_type, block.queue_length, block.rejections
            );
        } else if let Some(block) = block.as_any().downcast_ref::<DisposeBlock>() {
            println!(
                "Block: {:?} | Event: {:?} | Disposed Events: {:?}",
                block_id, event_type, block.disposed_events
            );
        }
    });
}
