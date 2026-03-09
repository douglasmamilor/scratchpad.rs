use std::time::{SystemTime, UNIX_EPOCH};

/// A simple xorshift64 pseudo-random number generator.
///
/// This is a fast, decent-quality PRNG suitable for games and graphics.
/// NOT cryptographically secure - do not use for security-sensitive applications.
pub struct Rng(u64);

impl Rng {
    /// Create a new RNG with the given seed.
    pub fn new(seed: u64) -> Self {
        // Ensure seed is non-zero (xorshift requires this)
        Self(if seed == 0 { 1 } else { seed })
    }

    /// Create a new RNG seeded from the current system time.
    pub fn from_time() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self::new(seed)
    }

    /// Generate the next random u64.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        // xorshift64 algorithm
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// Generate a random f32 in the range [0, 1).
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() as f64 / u64::MAX as f64) as f32
    }

    /// Generate a random f32 in the range [min, max].
    #[inline]
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::from_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_with_same_seed() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn different_seeds_produce_different_sequences() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(54321);

        // Very unlikely to match
        assert_ne!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    fn range_stays_within_bounds() {
        let mut rng = Rng::new(42);

        for _ in 0..1000 {
            let val = rng.range(10.0, 20.0);
            assert!(val >= 10.0 && val <= 20.0);
        }
    }

    #[test]
    fn zero_seed_handled() {
        let mut rng = Rng::new(0);
        // Should not panic or produce all zeros
        let val = rng.next_u64();
        assert_ne!(val, 0);
    }
}
