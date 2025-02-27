pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    // Seeds from system time if no seed provided
    pub fn new_from_time() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self::new(seed)
    }

    // Xorshift64 algorithm
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    // Generate number in range [0, max)
    pub fn gen_range(&mut self, max: u64) -> u64 {
        self.next_u64() % max
    }

    // Generate float in range [0.0, 1.0)
    pub fn gen_float(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_next() {
        let mut rng = SimpleRng::new_from_time();
        let val = rng.gen_float();
        assert!(val >= 0.0 && val <= 1.0);
    }

    #[test]
    fn test_rng_next_range() {
        let mut rng = SimpleRng::new_from_time();
        let max = 10;
        let val = rng.gen_range(max);
        assert!(val < max);
    }

    #[test]
    fn test_rng_sequence() {
        let mut rng = SimpleRng::new_from_time();
        let val1 = rng.gen_float();
        let val2 = rng.gen_float();
        assert!(val1 != val2, "RNG should generate different values");
    }
}
