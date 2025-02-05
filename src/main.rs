mod any;
mod blocks;
mod events;
mod system;

use crate::{
    blocks::{Block, CreateBlock, DisposeBlock, ProcessBlock},
    system::DiscreteEventSystem,
};
use rand_distr::Exp;
use std::time::{Duration, Instant};

fn average_queue_length(queue_lengths: &[(Instant, usize)]) -> f64 {
    let mut total = 0;
    let mut previous_time = queue_lengths[0].0;
    let mut previous_length = queue_lengths[0].1;
    for (time, length) in queue_lengths.iter().skip(1) {
        total += previous_length * (*time - previous_time).as_secs() as usize;
        previous_time = *time;
        previous_length = *length;
    }
    total as f64 / (previous_time - queue_lengths[0].0).as_secs_f64()
}

fn main() {
    let mut system = DiscreteEventSystem::new()
        .real_time()
        .add_block(
            CreateBlock::builder("create")
                .distribution(Exp::new(0.5).unwrap())
                .add_link("process1")
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process1")
                .distribution(Exp::new(1.0).unwrap())
                .max_queue_length(5)
                .add_link("process2")
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process2")
                .distribution(Exp::new(1.0).unwrap())
                .max_queue_length(5)
                .add_link("process3")
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process3")
                .distribution(Exp::new(1.0).unwrap())
                .max_queue_length(5)
                .add_link("dispose")
                .build(),
        )
        .add_block(DisposeBlock::new("dispose"));

    system.simulate(Duration::from_secs(15), |block, event_type| {
        if let Some(block) = block.as_any().downcast_ref::<CreateBlock<Exp<f64>>>() {
            println!(
                "Block: {:?} | Event: {:?} | Created Events: {:?}",
                block.id(),
                event_type,
                block.created_events
            );
        } else if let Some(block) = block.as_any().downcast_ref::<ProcessBlock<Exp<f64>>>() {
            println!(
                "Block: {:?} | Event: {:?} | Queue Length: {:?} | Rejections: {:?}",
                block.id(),
                event_type,
                block.queue_length,
                block.rejections
            );
        } else if let Some(block) = block.as_any().downcast_ref::<DisposeBlock>() {
            println!(
                "Block: {:?} | Event: {:?} | Disposed Events: {:?}",
                block.id(),
                event_type,
                block.disposed_events
            );
        }
    });

    println!("Final Simulation State:");
    for block in system.blocks.values() {
        if let Some(block) = block.as_any().downcast_ref::<CreateBlock<Exp<f64>>>() {
            println!(
                "Block: {:?} | Created Events: {:?}",
                block.id(),
                block.created_events
            );
        } else if let Some(block) = block.as_any().downcast_ref::<ProcessBlock<Exp<f64>>>() {
            println!(
                "Block: {:?} | Final Queue Length: {:?} | Total Rejections: {:?} | Queue Lengths: {:?}",
                block.id(),
                block.queue_length,
                block.rejections,
                average_queue_length(&block.queue_lengths)
            );
        } else if let Some(block) = block.as_any().downcast_ref::<DisposeBlock>() {
            println!(
                "Block: {:?} | Disposed Events: {:?}",
                block.id(),
                block.disposed_events
            );
        }
    }
}
