/// Trait for calculating a heuristic for the in-memory size of a struct.
pub trait InMemorySize {
    /// Returns a heuristic for the in-memory size of a struct.
    fn size(&self) -> usize;
}

impl InMemorySize for alloy_consensus::Header {
    fn size(&self) -> usize {
        self.size()
    }
}
