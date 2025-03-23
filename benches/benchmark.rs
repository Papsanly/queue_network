use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use queue_network::{
    blocks::{CreateBlock, DisposeBlock, ProcessBlock},
    network::QueueNetwork,
    queue::Queue,
    routers::DirectRouter,
};
use rand_distr::Normal;
use std::time::Duration;

fn get_parallel(size: u64) -> QueueNetwork {
    let mut network = QueueNetwork::new().add_block(
        CreateBlock::builder("create")
            .distribution(Normal::new(1.0, 0.1).unwrap())
            .router(DirectRouter::new("process1"))
            .build(),
    );
    for i in 0..size {
        network = network.add_block(
            ProcessBlock::builder(format!("process{}", i))
                .distribution(Normal::new(1.0, 0.1).unwrap())
                .queue(Queue::from_capacity(10))
                .router(DirectRouter::new("dispose"))
                .build(),
        );
    }
    network.add_block(DisposeBlock::new("dispose"))
}

fn get_sequential(size: u64) -> QueueNetwork {
    let mut network = QueueNetwork::new().add_block(
        CreateBlock::builder("create")
            .distribution(Normal::new(1.0, 0.1).unwrap())
            .router(DirectRouter::new("process1"))
            .build(),
    );
    for i in 0..size {
        network = network.add_block(
            ProcessBlock::builder(format!("process{}", i))
                .distribution(Normal::new(1.0, 0.1).unwrap())
                .queue(Queue::from_capacity(10))
                .router(DirectRouter::new(format!("process{}", i + 1)))
                .build(),
        );
    }
    network
        .add_block(
            ProcessBlock::builder(format!("process{}", size))
                .distribution(Normal::new(1.0, 0.1).unwrap())
                .queue(Queue::from_capacity(10))
                .router(DirectRouter::new("dispose"))
                .build(),
        )
        .add_block(DisposeBlock::new("dispose"))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_network");
    for size in (1..20).map(|x| 10 * x) {
        group.bench_with_input(BenchmarkId::new("sequential", size), &size, |b, &size| {
            let mut network = black_box(get_sequential(size));
            b.iter(|| network.simulate(Duration::from_secs(size)));
        });
        group.bench_with_input(BenchmarkId::new("parallel", size), &size, |b, &size| {
            let mut network = black_box(get_parallel(size));
            b.iter(|| network.simulate(Duration::from_secs(size)));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
