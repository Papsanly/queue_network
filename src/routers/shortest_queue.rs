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
        self.routes
            .iter()
            .min_by_key(|&block_id| {
                blocks
                    .get(block_id)
                    .expect("block id in shortest queue router should exist in blocks")
                    .queue()
                    .expect("block in shortest queue router should have a queue")
                    .length()
            })
            .cloned()
    }
}
