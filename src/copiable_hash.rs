use rustc_hash::FxHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const DEBUG_MAP_HASH_SIZE: usize = 1024 * 8;

/// A hash that can be copied and compared for equality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CopiableHash<E> {
    data: [Option<u64>; DEBUG_MAP_HASH_SIZE],
    len: usize,
    _marker: std::marker::PhantomData<E>,
}

impl<E> Default for CopiableHash<E> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<E: Hash + Eq> From<HashSet<E>> for CopiableHash<E> {
    fn from(set: HashSet<E>) -> Self {
        let mut hash = CopiableHash::empty();
        for key in set {
            hash.insert(key);
        }
        hash
    }
}

impl<E> CopiableHash<E> {
    /// Creates an empty hash.
    pub fn empty() -> Self {
        Self {
            data: [None; DEBUG_MAP_HASH_SIZE],
            len: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E: Hash> CopiableHash<E> {
    /// Adds a new element to the hash.
    pub fn insert(&mut self, key: E) -> bool {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        if self
            .data
            .iter()
            .take(self.len)
            .all(|x| x.map_or(true, |x| x != hash))
        {
            assert!(self.len < DEBUG_MAP_HASH_SIZE, "Cannot handle more than {} elements when checking. Please compile in release build or remove the `check_set` feature flag", DEBUG_MAP_HASH_SIZE);
            self.data[self.len] = Some(hash);
            self.len += 1;
            true
        } else {
            false
        }
    }

    /// Removes an element from the hash.
    pub fn remove(&mut self, key: &E) -> bool {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let pos = self
            .data
            .iter()
            .take(self.len)
            .position(|x| x.map_or(false, |x| x == hash));

        if let Some(pos) = pos {
            self.data.swap(pos, self.len - 1);
            self.len -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn random_test_with_hashset() {
        let mut reference = std::collections::HashSet::new();
        let mut target = CopiableHash::empty();

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let is_insert = rng.gen_bool(0.5);
            let key = rng.gen::<u64>();

            if is_insert {
                assert_eq!(reference.insert(key), target.insert(key));
            } else {
                assert_eq!(reference.remove(&key), target.remove(&key));
            }
        }
    }

    #[test]
    #[should_panic]
    fn capacity_over_test() {
        let mut target = CopiableHash::empty();
        for i in 0..DEBUG_MAP_HASH_SIZE {
            assert!(target.insert(i));
        }
        target.insert(DEBUG_MAP_HASH_SIZE);
    }
}
