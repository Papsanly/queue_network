use crate::{
    blocks::{Block, BlockId},
    routers::Router,
};
use std::collections::HashMap;

pub struct DirectRouter {
    next: BlockId,
}

impl DirectRouter {
    pub fn new(next: BlockId) -> Self {
        Self { next }
    }
}

impl Router for DirectRouter {
    fn next(&self, _blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        Some(self.next)
    }
}
