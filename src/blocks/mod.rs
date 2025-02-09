use create::CreateBlock as Create;
use dispose::DisposeBlock as Dispose;
use process::ProcessBlock as Process;

pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::events::Event;
use rand::{distr::Uniform, Rng};
use rand_distr::{Exp, Normal};
use std::{collections::BinaryHeap, fmt::Debug, time::Instant};

mod create;
mod dispose;
mod process;
mod queue;

pub type BlockId = &'static str;

pub trait Block {
    type Stats;
    fn id(&self) -> BlockId;
    fn links(&self) -> &[BlockId];
    fn stats(&self) -> Self::Stats;
    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
}

macro_rules! impl_distribution {
    ($($name:ident),*) => {
        pub enum Distribution {
            $(
                $name($name<f32>),
            )*
        }

        $(
            impl From<$name<f32>> for Distribution {
                fn from(distribution: $name<f32>) -> Self {
                    Distribution::$name(distribution)
                }
            }
        )*

        impl rand_distr::Distribution<f32> for Distribution {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
                match self {
                    $(
                        Distribution::$name(distribution) => distribution.sample(rng),
                    )*
                }
            }
        }
    };
}

impl_distribution!(Exp, Normal, Uniform);

macro_rules! impl_block {
    ($($name:ident),*) => {
        pub enum BlockType {
            $($name($name),)*
        }

        $(
            impl From<$name> for BlockType {
                fn from(block: $name) -> Self {
                    BlockType::$name(block)
                }
            }
        )*

        impl Block for BlockType {
            type Stats = Box<dyn Debug>;

            fn id(&self) -> BlockId {
                match self {
                    $(BlockType::$name(block) => block.id(),)*
                }
            }

            fn links(&self) -> &[BlockId] {
                match self {
                    $(BlockType::$name(block) => block.links(),)*
                }
            }

            fn stats(&self) -> Box<dyn Debug> {
                match self {
                    $(BlockType::$name(block) => Box::new(block.stats()),)*
                }
            }

            fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $(BlockType::$name(block) => block.init(event_queue, current_time),)*
                }
            }

            fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $(BlockType::$name(block) => block.process_in(event_queue, current_time),)*
                }
            }

            fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $(BlockType::$name(block) => block.process_out(event_queue, current_time),)*
                }
            }
        }
    };
}

impl_block!(Create, Dispose, Process);
