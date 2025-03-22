mod regular;
mod shared;

pub use regular::RegularQueue;
pub use shared::SharedQueuePool;
use std::time::Duration;

pub trait Queue {
    fn length(&self) -> usize;
    fn capacity(&self) -> Option<usize>;
    fn enqueue(&mut self, simulation_duration: Duration);
    fn dequeue(&mut self, simulation_duration: Duration);
    fn total_weighted_time(&self) -> f32;
    fn duration(&self) -> Duration;
    fn average_length(&self) -> f32;
}
