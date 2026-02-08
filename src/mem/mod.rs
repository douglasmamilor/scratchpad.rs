pub use align::align_up_pow2;

mod align {
    /// Align n up to the next multiple of align, where align is a power of 2.
    pub fn align_up_pow2(n: usize, align: usize) -> usize {
        assert!(align.is_power_of_two(), "Must be a power of 2");
        (n + (align - 1)) & !(align - 1)
    }
}
