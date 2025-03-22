mod create;
mod dispose;
mod process;

pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::{events::Event, queues::Queue};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    time::Duration,
};

pub type BlockId = &'static str;

pub trait Block {
    fn id(&self) -> BlockId;
    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId>;
    fn step_stats(&self) -> Box<dyn Debug>;
    fn stats(&self) -> Box<dyn Debug>;
    fn queue(&self) -> Option<&dyn Queue> {
        None
    }
    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>) {}
    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _simulation_duration: Duration) {
    }
    fn process_out(
        &mut self,
        _event_queue: &mut BinaryHeap<Event>,
        _simulation_duration: Duration,
    ) {
    }
}
