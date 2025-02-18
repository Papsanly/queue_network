mod direct;
mod probability;
mod shortest_queue;

use crate::blocks::{Block, BlockId};
pub use direct::DirectRouter;
pub use probability::ProbabilityRouter;
pub use shortest_queue::ShortestQueueRouter;
use shortest_queue::ShortestQueueRouter as ShortestQueue;
use std::collections::HashMap;

pub trait Router {
    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId>;
}
