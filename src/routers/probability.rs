use crate::{
    blocks::{Block, BlockId},
    routers::Router,
};
use rand::{rng, Rng};
use std::collections::HashMap;

pub struct ProbabilityRouter {
    next: Vec<(f32, BlockId)>,
}

impl ProbabilityRouter {
    pub fn new(next: &[(f32, BlockId)]) -> Self {
        let total = next.iter().map(|(p, _)| p).sum::<f32>();
        Self {
            next: next
                .iter()
                .map(|(p, id)| (*p / total, id.clone()))
                .collect(),
        }
    }
}

impl Router for ProbabilityRouter {
    fn next(&self, _blocks: &HashMap<BlockId, Box<dyn Block>>) -> Option<BlockId> {
        let random = rng().random::<f32>();

        let mut sum = 0.0;
        for (probability, block_id) in &self.next {
            sum += probability;
            if random < sum {
                return Some(block_id.clone());
            }
        }

        None
    }
}
