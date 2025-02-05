pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::{any::AsAny, events::Event};
use std::{collections::BinaryHeap, time::Instant};

mod create;
mod dispose;
mod process;

pub type BlockId = &'static str;

pub trait Block: AsAny {
    fn id(&self) -> BlockId;
    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
}
