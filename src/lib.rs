use rustc_hash::FxHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Default, Copy, Clone, Debug)]
pub struct ZobristHash<E> {
    hash: u64,
    _data: std::marker::PhantomData<E>,
    #[cfg(debug_assertions)]
    checker: Option<*mut HashSet<E>>,
}

impl<E> ZobristHash<E> {
    pub fn empty() -> Self {
        let hashset = Box::new(HashSet::new());
        let ptr = Box::into_raw(hashset);
        Self {
            hash: 0,
            _data: std::marker::PhantomData,
            #[cfg(debug_assertions)]
            checker: Some(ptr),
        }
    }
}

impl<E> From<u64> for ZobristHash<E> {
    fn from(hash: u64) -> Self {
        Self {
            hash,
            _data: std::marker::PhantomData,
            #[cfg(debug_assertions)]
            checker: None,
        }
    }
}

impl<E> From<ZobristHash<E>> for u64 {
    fn from(hash: ZobristHash<E>) -> u64 {
        hash.hash
    }
}

#[cfg(not(debug_assertions))]
impl<E: Hash + Clone> ZobristHash<E> {
    pub fn add(&mut self, key: &E) {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        self.hash ^= hasher.finish();
    }

    pub fn remove(&mut self, key: &E) {
        self.add(key);
    }
}

#[cfg(debug_assertions)]
impl<E: Hash + Eq + Clone> ZobristHash<E> {
    pub fn add(&mut self, key: &E) {
        let hashset = unsafe { self.checker.map(|x| x.as_mut()).unwrap() };
        assert!(hashset.map(|x| x.insert(key.clone())).unwrap_or(true));
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        self.hash ^= hasher.finish();
    }

    pub fn remove(&mut self, key: &E) {
        let hashset = unsafe { self.checker.map(|x| x.as_mut()).unwrap() };
        assert!(hashset.map(|x| x.remove(&key.clone())).unwrap_or(true));

        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        self.hash ^= hasher.finish();
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

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_zobrist_hash_double_add_debug() {
        let mut hash = ZobristHash::empty();
        let key = 42;
        hash.add(&key);
        hash.add(&key);
    }
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_zobrist_hash_empty_remove_debug() {
        let mut hash = ZobristHash::empty();
        let key = 42;
        hash.remove(&key);
    }
}
