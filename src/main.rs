mod blocks;
mod devices;
mod events;
mod network;
mod queues;
mod routers;
mod stats;
mod weighted_average;

use crate::{
    blocks::{CreateBlock, DisposeBlock, ProcessBlock},
    events::Event,
    network::QueueNetwork,
    queues::{RegularQueue, SharedQueuePool},
    routers::{DirectRouter, ShortestQueueRouter},
};
use rand_distr::{Exp, Normal};
use std::time::Duration;

fn main() {
    let shared_queue_pool = SharedQueuePool::new()
        .add_queue("process1", RegularQueue::from_capacity(3))
        .add_queue("process2", RegularQueue::from_capacity(3));

    let mut network = QueueNetwork::new()
        .add_block(
            CreateBlock::builder("create")
                .distribution(Exp::new(1.0 / 0.5).unwrap())
                .router(ShortestQueueRouter::new(&["process1", "process2"]))
                .build(),
        )
        .add_block({
            let mut res = ProcessBlock::builder("process1")
                .distribution(Normal::new(1.0, 0.3).unwrap())
                .queue(shared_queue_pool.get("process1"))
                .router(DirectRouter::new("dispose"))
                .build();
            res.devices.load(Duration::ZERO);
            for _ in 0..2 {
                res.queue.as_mut().unwrap().enqueue(Duration::ZERO);
            }
            res
        })
        .add_block({
            let mut res = ProcessBlock::builder("process2")
                .distribution(Normal::new(1.0, 0.3).unwrap())
                .queue(shared_queue_pool.get("process2"))
                .router(DirectRouter::new("dispose"))
                .build();
            res.devices.load(Duration::ZERO);
            for _ in 0..2 {
                res.queue.as_mut().unwrap().enqueue(Duration::ZERO);
            }
            res
        })
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
