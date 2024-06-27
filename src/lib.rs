use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

///
#[derive(Default, Copy, Clone, Debug)]
pub struct ZobristHash<K> {
    hash: u64,
    _data: std::marker::PhantomData<K>,
}

impl<K> ZobristHash<K> {
    pub fn empty() -> Self {
        Self {
            hash: 0,
            _data: std::marker::PhantomData,
        }
    }
}

impl<K> From<u64> for ZobristHash<K> {
    fn from(hash: u64) -> Self {
        Self {
            hash,
            _data: std::marker::PhantomData,
        }
    }
}

impl<K> From<ZobristHash<K>> for u64 {
    fn from(hash: ZobristHash<K>) -> u64 {
        hash.hash
    }
}

impl<K: Hash + Clone> ZobristHash<K> {
    pub fn add(&mut self, key: &K) {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        self.hash ^= hasher.finish();
    }

    pub fn remove(&mut self, key: &K) {
        self.add(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_hash() {
        let mut hash = ZobristHash::empty();
        let key = 42;
        hash.add(&key);

        assert_ne!(hash.hash, 0);
        hash.remove(&key);
        assert_eq!(hash.hash, 0);
    }

    #[test]
    fn test_zobrist_hash2() {
        let mut hash = ZobristHash::empty();
        let key = (1, 42);
        hash.add(&key);
        let hv = hash.hash;
        let key = (2, 42);
        hash.add(&key);
        assert_ne!(hash.hash, hv);
    }

    #[test]
    fn test_zobrist_hash3() {
        let mut hash1 = ZobristHash::empty();
        let key = (1, 42);
        hash1.add(&key);
        let mut hash2 = ZobristHash::empty();
        let key = (2, 42);
        hash2.add(&key);
        assert_ne!(hash1.hash, hash2.hash);
    }
}
