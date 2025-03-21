use crate::{
    blocks::{Block, BlockId},
    routers::Router,
};
use std::collections::HashMap;

pub struct ShortestQueueRouter {
    routes: Vec<BlockId>,
}

impl ShortestQueueRouter {
    pub fn new(queues: &[BlockId]) -> Self {
        Self {
            routes: queues.to_vec(),
        }
    }
}

impl Router for ShortestQueueRouter {
    fn next(&self, blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        blocks
            .iter()
            .filter(|(block_id, _)| self.routes.contains(block_id))
            .min_by_key(|(_, block)| block.queue().map(|q| q.length).unwrap_or(0))
            .map(|(block_id, _)| *block_id)
    }
}
