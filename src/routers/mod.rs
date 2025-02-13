mod direct;
mod probability;
mod shortest_queue;

use crate::blocks::{BlockId, BlockType};
use direct::DirectRouter as Direct;
pub use direct::DirectRouter;
use probability::ProbabilityRouter as Probability;
pub use probability::ProbabilityRouter;
use shortest_queue::ShortestQueueRouter as ShortestQueue;
pub use shortest_queue::ShortestQueueRouter;
use std::collections::HashMap;

pub trait Router {
    fn next(&self, blocks: &HashMap<BlockId, BlockType>) -> Option<BlockId>;
}

macro_rules! impl_router {
    ($enum_name:ident {$($name:ident),*}) => {
        pub enum $enum_name {
            $($name($name),)*
        }

        $(
            impl From<$name> for $enum_name {
                fn from(router: $name) -> Self {
                    RouterType::$name(router)
                }
            }
        )*

        impl Router for $enum_name {
            fn next(&self, blocks: &HashMap<BlockId, BlockType>) -> Option<BlockId> {
                match self {
                    $($enum_name::$name(router) => router.next(blocks),)*
                }
            }
        }
    };
}

impl_router!(RouterType {
    Direct,
    Probability,
    ShortestQueue
});
