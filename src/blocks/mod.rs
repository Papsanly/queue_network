pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::Event;
use std::{any::Any, collections::BinaryHeap, time::Instant};

mod create;
mod dispose;
mod process;

pub trait Block<BlockId> {
    fn init(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant);
    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant);
    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event<BlockId>>, current_time: Instant);
    fn as_any(&self) -> &dyn Any;
}
