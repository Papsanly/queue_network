use rand::{distr::Distribution, Rng};

pub struct Deterministic {
    value: f32,
}

impl Deterministic {
    pub fn new(value: f32) -> Deterministic {
        Deterministic { value }
    }
}

impl Distribution<f32> for Deterministic {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> f32 {
        self.value
    }
}
