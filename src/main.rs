mod blocks;
mod devices;
mod distributions;
mod events;
mod network;
mod queues;
mod routers;
mod stats;
mod weighted_average;

use crate::{
    blocks::{CreateBlock, DisposeBlock, ProcessBlock},
    devices::Devices,
    events::Event,
    network::QueueNetwork,
    queues::{Queue, RegularQueue, SharedQueuePool},
    routers::{DirectRouter, ShortestQueueRouter},
    stats::Stats,
};
use rand_distr::{Exp, Normal};
use std::time::Duration;

fn main() {
    let shared_queue_pool = SharedQueuePool::new()
        .add_queue("process1", {
            let mut res = RegularQueue::from_capacity(3);
            for i in 1..3 {
                res.enqueue(i, Duration::ZERO);
            }
            res
        })
        .add_queue("process2", {
            let mut res = RegularQueue::from_capacity(3);
            for i in 4..6 {
                res.enqueue(i, Duration::ZERO);
            }
            res
        });

    let mut network = QueueNetwork::new()
        .add_block(
            CreateBlock::builder("create")
                .distribution(Exp::new(1.0 / 0.5).unwrap())
                .router(ShortestQueueRouter::new(&["process1", "process2"]))
                .first_at((6, Duration::from_secs_f32(0.1)))
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process1")
                .distribution(Normal::new(1.0, 0.3).unwrap())
                .queue(shared_queue_pool.get("process1"))
                .devices({
                    let mut res = Devices::new(1);
                    res.load(0, Duration::ZERO);
                    res
                })
                .router(DirectRouter::new("dispose"))
                .build(),
        )
        .add_block(
            ProcessBlock::builder("process2")
                .distribution(Normal::new(1.0, 0.3).unwrap())
                .queue(shared_queue_pool.get("process2"))
                .devices({
                    let mut res = Devices::new(1);
                    res.load(3, Duration::ZERO);
                    res
                })
                .router(DirectRouter::new("dispose"))
                .build(),
        )
        .add_block(DisposeBlock::new("dispose"))
        .on_simulation_step(|network, Event(time, block_id, event_type, id)| {
            let block = network.blocks.get(block_id).unwrap();
            println!(
                "Elapsed Time: {:.3} | Event: {:?} | Id: {} | {}: {:#?}",
                time.as_secs_f32(),
                event_type,
                id,
                block_id,
                block.step_stats()
            );
        });
    network.event_count = 6;

    network.simulate(Duration::from_secs(1000));

    println!("\n==== Final Simulation State ====\n");
    let mut blocks = network.blocks.values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| match block.id() {
        "create" => "0".to_string(),
        "dispose" => "2".to_string(),
        _ => "1".to_string() + block.id(),
    });
    for block in blocks {
        println!("{}: {:#?}", block.id(), block.stats());
    }
    println!("network: {:#?}", network.stats());
}
