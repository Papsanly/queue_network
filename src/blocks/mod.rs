use create::CreateBlock as Create;
use dispose::DisposeBlock as Dispose;
use process::ProcessBlock as Process;

pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::events::Event;
use rand::Rng;
use rand_distr::{Exp, Normal, Uniform};
use std::{collections::BinaryHeap, fmt::Debug, time::Instant};

mod create;
mod dispose;
mod process;
mod queue;

pub type BlockId = &'static str;

pub trait Block {
    type Stats;
    fn id(&self) -> BlockId;
    fn next(&self) -> Option<BlockId>;
    fn stats(&self) -> Self::Stats;
    fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
    fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant);
}

macro_rules! impl_distribution {
    ($enum_name:ident {$($name:ident),*}) => {
        pub enum $enum_name {
            $($name($name<f32>),)*
        }

        $(
            impl From<$name<f32>> for $enum_name {
                fn from(distribution: $name<f32>) -> Self {
                    DistributionType::$name(distribution)
                }
            }
        )*

        impl rand_distr::Distribution<f32> for $enum_name {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
                match self {
                    $($enum_name::$name(distribution) => distribution.sample(rng),)*
                }
            }
        }
    };
}

impl_distribution!(DistributionType {
    Exp,
    Normal,
    Uniform
});

macro_rules! impl_block {
    ($enum_name:ident {$($name:ident),*}) => {
        pub enum $enum_name {
            $($name($name),)*
        }

        $(
            impl From<$name> for $enum_name {
                fn from(block: $name) -> Self {
                    $enum_name::$name(block)
                }
            }
        )*

        impl Block for $enum_name {
            type Stats = Box<dyn Debug>;

            fn id(&self) -> BlockId {
                match self {
                    $($enum_name::$name(block) => block.id(),)*
                }
            }

            fn next(&self) -> Option<BlockId> {
                match self {
                    $($enum_name::$name(block) => block.next(),)*
                }
            }

            fn stats(&self) -> Box<dyn Debug> {
                match self {
                    $($enum_name::$name(block) => Box::new(block.stats()),)*
                }
            }

            fn init(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $($enum_name::$name(block) => block.init(event_queue, current_time),)*
                }
            }

            fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $($enum_name::$name(block) => block.process_in(event_queue, current_time),)*
                }
            }

            fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, current_time: Instant) {
                match self {
                    $($enum_name::$name(block) => block.process_out(event_queue, current_time),)*
                }
            }
        }
    };
}

impl_block!(BlockType {
    Create,
    Dispose,
    Process
});
