mod create;
mod dispose;
mod process;

pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::{
    events::Event,
    queues::Queue,
    stats::{Stats, StepStats},
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::Duration,
};

pub type BlockId = &'static str;

pub trait Block: Stats + StepStats {
    fn id(&self) -> BlockId;
    fn kind(&self) -> &'static str;
    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId>;
    fn queue(&self) -> Option<&dyn Queue> {
        None
    }
    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>) {}
    fn process_in(
        &mut self,
        _event_id: usize,
        _event_queue: &mut BinaryHeap<Event>,
        _simulation_duration: Duration,
    ) -> bool {
        false
    }
    fn process_out(
        &mut self,
        _event_id: usize,
        _event_queue: &mut BinaryHeap<Event>,
        _simulation_duration: Duration,
    ) {
    }
}
