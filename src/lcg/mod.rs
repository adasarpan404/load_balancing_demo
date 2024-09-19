use std::time::{SystemTime, UNIX_EPOCH};

pub struct LcgRng {
    seed: u64,
}

impl LcgRng {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let seed = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self { seed }
    }

    pub fn lcg_rand(&mut self) -> f64 {
        const A: u64 = 1664525;
        const C: u64 = 10139004223;
        const M: u64 = 100;
        self.seed = (A * self.seed + C) % M;
        let normalized_value = self.seed as f64 / M as f64;
        normalized_value * (2.0 - 0.1) + 0.1
    }
}
