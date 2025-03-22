mod blocks;
mod devices;
mod distributions;
mod events;
mod network;
mod queue;
mod routers;
mod stats;
mod weighted_average;

use crate::{
    blocks::{CreateBlock, DisposeBlock, ProcessBlock},
    devices::Devices,
    events::Event,
    network::QueueNetwork,
    queue::Queue,
    routers::{DirectRouter, ProbabilityRouter},
};
use rand_distr::Exp;
use std::time::Duration;

fn main() {
    let mut network = QueueNetwork::new()
        .add_block(
            CreateBlock::builder("create")
                .distribution(Exp::new(0.5).unwrap())
                .router(DirectRouter::new("process1"))
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process1")
                .distribution(Exp::new(1.0).unwrap())
                .queue(Queue::from_capacity(5))
                .router(DirectRouter::new("process2"))
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process2")
                .distribution(Exp::new(1.0).unwrap())
                .queue(Queue::from_capacity(5))
                .devices(Devices::new(2))
                .router(DirectRouter::new("process3"))
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process3")
                .distribution(Exp::new(1.0).unwrap())
                .queue(Queue::from_capacity(5))
                .router(ProbabilityRouter::new(&[
                    (0.5, "process2"),
                    (0.5, "dispose"),
                ]))
                .build(),
        )
        .add_block(DisposeBlock::new("dispose"))
        .on_simulation_step(|network, Event(time, block_id, event_type)| {
            let block = network.blocks.get(block_id).unwrap();
            println!(
                "Elapsed Time: {:.3} | Event: {:?} {}: {:#?}",
                time.as_secs_f32(),
                event_type,
                block_id,
                block.step_stats()
            );
        });

    network.simulate(Duration::from_secs(1000));

    println!("\nFinal Simulation State:");
    let mut blocks = network.blocks.values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| match block.id() {
        "create" => "0".to_string(),
        "dispose" => "2".to_string(),
        _ => "1".to_string() + block.id(),
    });
    for block in blocks {
        println!("{}: {:#?}", block.id(), block.stats());
    }
}
