use crate::{
    blocks::{Block, BlockId},
    routers::Router,
};
use std::collections::HashMap;

pub struct DirectRouter {
    next: BlockId,
}

impl DirectRouter {
    pub fn new(next: impl Into<BlockId>) -> Self {
        Self { next: next.into() }
    }
}

impl Router for DirectRouter {
    fn next(&self, _blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        Some(self.next.clone())
    }
}
