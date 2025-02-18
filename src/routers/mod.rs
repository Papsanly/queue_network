mod direct;
mod probability;

use crate::blocks::{Block, BlockId};
pub use direct::DirectRouter;
pub use probability::ProbabilityRouter;
use std::collections::HashMap;

pub trait Router {
    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId>;
}
