//! A growable bitset, for per index flags kept off the hot claim path.

/// A growable bitset indexed by `usize`, i.e. by [`bevy_ecs::component::ComponentId::index`].
///
/// Indices past the end are not contained.
#[derive(Clone, Debug, Default)]
pub(crate) struct BitSet(Vec<u64>);

impl BitSet {
    /// Adds the index, growing the set if needed
    pub(crate) fn insert(&mut self, index: usize) {
        let (word, bit) = (index / 64, index % 64);
        if word >= self.0.len() {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= 1 << bit;
    }

    /// Returns true if the index was inserted
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        self.0
            .get(index / 64)
            .is_some_and(|word| word & (1 << (index % 64)) != 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_set_contains_only_inserted_indices() {
        let mut set = BitSet::default();
        set.insert(1);
        set.insert(130);
        assert!(set.contains(1) && set.contains(130));
        assert!(!set.contains(0) && !set.contains(64) && !set.contains(1000));
    }
}
