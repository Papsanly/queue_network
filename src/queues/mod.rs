mod regular;
mod shared;

use crate::stats::{Stats, StepStats};
pub use regular::RegularQueue;
pub use shared::SharedQueuePool;
use std::time::Duration;

pub trait Queue: StepStats + Stats {
    fn length(&self) -> usize;
    fn weighted_total(&self) -> f32;
    fn capacity(&self) -> Option<usize>;
    fn enqueue(&mut self, event_id: usize, simulation_duration: Duration);
    fn dequeue(&mut self, simulation_duration: Duration) -> usize;
}
