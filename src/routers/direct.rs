use crate::{
    blocks::{BlockId, BlockType},
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
    fn next(&self, _blocks: &HashMap<BlockId, BlockType>) -> Option<BlockId> {
        Some(self.next)
    }
}
