pub use align::align_up_pow2;

mod align {
    /// Align n up to the next multiple of align, where align is a power of 2.
    /// It works because you are always at most align-1 bits away from the next multiple of align,
    /// so adding align-1 will push you at least to the next multiple potentially with an overage,
    /// and then masking off the lower bits will, remove the excess and give you the aligned address.
    ///
    /// e.g 10 is 2 bits away from the next multiple of 4 (12), so adding 3 (align-1) gives you 13,
    /// which is has a 1 bit excess that is then masked off.
    /// Likewise, 12 is already a multiple of 4, so adding 3 gives you 15, which has a 3 bit excess
    /// that is masked off.
    pub fn align_up_pow2(n: usize, align: usize) -> usize {
        assert!(align.is_power_of_two(), "Must be a power of 2");
        (n + (align - 1)) & !(align - 1)
    }
}
