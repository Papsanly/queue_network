use crate::{
    blocks::{BlockId, BlockType},
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
    fn next(&self, blocks: &HashMap<BlockId, BlockType>) -> Option<BlockId> {
        blocks
            .iter()
            .filter(|(block_id, _)| self.routes.contains(block_id))
            .min_by_key(|(_, block)| {
                let BlockType::Process(block) = block else {
                    panic!("ShortestQueueRouter only works with Process blocks");
                };
                block.queue.as_ref().map(|q| q.length).unwrap_or(0)
            })
            .map(|(block_id, _)| *block_id)
    }
}
