use std::fmt::Debug;

pub trait Stats {
    fn stats(&self) -> Box<dyn Debug>;
}

pub trait StepStats {
    fn step_stats(&self) -> Box<dyn Debug>;
}
