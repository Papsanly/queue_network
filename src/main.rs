mod blocks;
mod events;
mod system;

use crate::{
    blocks::{BlockTrait, CreateBlock, DisposeBlock, ProcessBlock},
    system::QueueNetwork,
};
use rand_distr::Exp;
use std::time::{Duration, Instant};

fn main() {
    let mut network = QueueNetwork::new()
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
    network.simulate(Duration::from_secs(1000), |instant, block, event_type| {
        println!(
            "Elapsed Time: {:.3} | Event: {:?} | Block: {} | {:?}",
            (instant - start_time).as_secs_f32(),
            event_type,
            block.id(),
            block.stats()
        );
    });

    println!("Final Simulation State:");
    let mut blocks = network.blocks.values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| match block.id() {
        "create" => "0".to_string(),
        "dispose" => "2".to_string(),
        _ => "1".to_string() + block.id(),
    });
    for block in blocks {
        println!("Block: {} | {:?}", block.id(), block.stats());
    }
}
