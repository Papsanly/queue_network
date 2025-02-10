use crate::{blocks::BlockId, routers::Router};

pub struct DirectRouter {
    next: BlockId,
}

impl DirectRouter {
    pub fn new(next: BlockId) -> Self {
        Self { next }
    }
}

impl Router for DirectRouter {
    fn next(&self) -> Option<BlockId> {
        Some(self.next)
    }
}
