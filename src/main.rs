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
                "Elapsed Time: {:.3} | Block: {:?} | Final Queue Length: {}  | Total Processed: {} | Total Rejections: {} | Rejection Probability: {:.3}",
                elapsed_seconds,
                block.id(),
                block.queue.length,
                block.processed,
                block.rejections,
                block.rejections as f64 / block.processed as f64,
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

    let create_block = system
        .blocks
        .get("create")
        .unwrap()
        .as_any()
        .downcast_ref::<CreateBlock<Exp<f64>>>()
        .unwrap();
    let mut process_blocks = system
        .blocks
        .values()
        .filter_map(|block| block.as_any().downcast_ref::<ProcessBlock<Exp<f64>>>())
        .collect::<Vec<_>>();
    process_blocks.sort_by_key(|block| block.id());
    let dispose_block = system
        .blocks
        .get("dispose")
        .unwrap()
        .as_any()
        .downcast_ref::<DisposeBlock>()
        .unwrap();

    println!(
        "Block: {:?}\n\tCreated Events: {:?}",
        create_block.id(),
        create_block.created_events
    );
    for block in process_blocks {
        println!(
            "Block: {:?}\n\tFinal Queue Length: {}\n\tTotal Processed: {}\n\tTotal Rejections: {}\n\tAverage Queue Lengths: {:.3}\n\tAverage Wait Time {:.3}\n\tRejection Probability: {:.3}",
            block.id(),
            block.queue.length,
            block.processed,
            block.rejections,
            block.queue.average_length(),
            block.queue.total_weighted_time().0 / block.processed as f64,
            block.rejections as f64 / block.processed as f64,
        );
    }
    println!(
        "Block: {:?}\n\tDisposed Events: {:?}",
        dispose_block.id(),
        dispose_block.disposed_events
    );
}
