use create::CreateBlock as Create;
use dispose::DisposeBlock as Dispose;
use process::ProcessBlock as Process;

pub use create::CreateBlock;
pub use dispose::DisposeBlock;
pub use process::ProcessBlock;

use crate::events::Event;
use rand::Rng;
use rand_distr::{Exp, Normal, Uniform};
use std::{collections::BinaryHeap, fmt::Debug, time::Duration};

mod create;
mod dispose;
mod process;

pub type BlockId = &'static str;

pub trait Block {
    type StepStats;
    type Stats;
    fn id(&self) -> BlockId;
    fn next(&self) -> Option<BlockId>;
    fn step_stats(&self) -> Self::StepStats;
    fn stats(&self) -> Self::Stats;
    fn init(&mut self, _event_queue: &mut BinaryHeap<Event>) {}
    fn process_in(&mut self, _event_queue: &mut BinaryHeap<Event>, _simulation_duration: Duration) {
    }
    fn process_out(
        &mut self,
        _event_queue: &mut BinaryHeap<Event>,
        _simulation_duration: Duration,
    ) {
    }
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
            type StepStats = Box<dyn Debug>;
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

            fn step_stats(&self) -> Box<dyn Debug> {
                match self {
                    $($enum_name::$name(block) => Box::new(block.step_stats()),)*
                }
            }

            fn stats(&self) -> Box<dyn Debug> {
                match self {
                    $($enum_name::$name(block) => Box::new(block.stats()),)*
                }
            }

            fn init(&mut self, event_queue: &mut BinaryHeap<Event>) {
                match self {
                    $($enum_name::$name(block) => block.init(event_queue),)*
                }
            }

            fn process_in(&mut self, event_queue: &mut BinaryHeap<Event>, simulation_duration: Duration) {
                match self {
                    $($enum_name::$name(block) => block.process_in(event_queue, simulation_duration),)*
                }
            }

            fn process_out(&mut self, event_queue: &mut BinaryHeap<Event>, simulation_duration: Duration) {
                match self {
                    $($enum_name::$name(block) => block.process_out(event_queue, simulation_duration),)*
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
