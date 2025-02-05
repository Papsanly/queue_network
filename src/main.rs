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

fn main() {
    let mut system = DiscreteEventSystem::new()
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

    let start_time = Instant::now();
    system.simulate(Duration::from_secs(1000), |instant, block, event_type| {
        let elapsed_seconds = (instant - start_time).as_secs_f64();
        if let Some(block) = block.as_any().downcast_ref::<CreateBlock<Exp<f64>>>() {
            println!(
                "Elapsed Time: {:.3} | Block: {:?} | Event: {:?} | Created Events: {:?}",
                elapsed_seconds,
                block.id(),
                event_type,
                block.created_events
            );
        } else if let Some(block) = block.as_any().downcast_ref::<ProcessBlock<Exp<f64>>>() {
            println!(
                "Elapsed Time: {:.3} | Block: {:?} | Event: {:?} | Queue Length: {:?} | Rejections: {:?}",
                elapsed_seconds,
                block.id(),
                event_type,
                block.queue.length,
                block.rejections
            );
        } else if let Some(block) = block.as_any().downcast_ref::<DisposeBlock>() {
            println!(
                "Elapsed Time: {:.3} | Block: {:?} | Event: {:?} | Disposed Events: {:?}",
                elapsed_seconds,
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
                block.queue.length,
                block.rejections,
                block.queue.average_length()
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
