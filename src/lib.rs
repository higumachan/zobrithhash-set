mod copiable_hash;

#[cfg(all(debug_assertions, feature = "check_set_behavior"))]
use crate::copiable_hash::CopiableHash;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

/// Implementation of [Zobrist hashing](https://en.wikipedia.org/wiki/Zobrist_hashing)
///
/// This Zobrist hash implementation does not use a table to maintain a context-less design. `FxHash` is sufficiently fast, but if you want to achieve even higher speeds, consider implementing a version that uses a table.
///
/// An example implementation of a hash representing a chessboard is shown below
/// ```rust
/// use zobristhash_set::ZobristHashSet;
///
/// #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
/// enum Piece {
///     WhitePawn,
///     WhiteKnight,
///     WhiteBishop,
///     WhiteRook,
///     WhiteQueen,
///     WhiteKing,
///     BlackPawn,
///     BlackKnight,
///     BlackBishop,
///     BlackRook,
///     BlackQueen,
///     BlackKing,
/// }
///
/// const BOARD_SIZE: usize = 8;
///
/// #[derive(Debug)]
/// struct ChessBoard {
///     board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
///     zobrist: ZobristHashSet<(usize, usize, Piece)>,
/// }
///
/// impl ChessBoard {
///     pub fn new() -> Self {
///         ChessBoard {
///             board: [[None; BOARD_SIZE]; BOARD_SIZE],
///             zobrist: ZobristHashSet::empty(),
///         }
///     }
///
///     pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
///         if let Some(old_piece) = self.board[x][y] {
///             self.zobrist.remove(&(x, y, old_piece));
///         }
///         if let Some(new_piece) = piece {
///             self.zobrist.add(&(x, y, new_piece));
///         }
///         self.board[x][y] = piece;
///     }
///
///     pub fn initialize(&mut self) {
/// #         self.set_piece(0, 0, Some(Piece::WhiteRook));
/// #         self.set_piece(0, 1, Some(Piece::WhiteKnight));
/// #         self.set_piece(0, 2, Some(Piece::WhiteBishop));
/// #         self.set_piece(0, 3, Some(Piece::WhiteQueen));
/// #         self.set_piece(0, 4, Some(Piece::WhiteKing));
/// #         self.set_piece(0, 5, Some(Piece::WhiteBishop));
/// #         self.set_piece(0, 6, Some(Piece::WhiteKnight));
/// #         self.set_piece(0, 7, Some(Piece::WhiteRook));
/// #         for i in 0..BOARD_SIZE {
/// #             self.set_piece(1, i, Some(Piece::WhitePawn));
/// #         }
/// #
/// #         self.set_piece(7, 0, Some(Piece::BlackRook));
/// #         self.set_piece(7, 1, Some(Piece::BlackKnight));
/// #         self.set_piece(7, 2, Some(Piece::BlackBishop));
/// #         self.set_piece(7, 3, Some(Piece::BlackQueen));
/// #         self.set_piece(7, 4, Some(Piece::BlackKing));
/// #         self.set_piece(7, 5, Some(Piece::BlackBishop));
/// #         self.set_piece(7, 6, Some(Piece::BlackKnight));
/// #         self.set_piece(7, 7, Some(Piece::BlackRook));
/// #         for i in 0..BOARD_SIZE {
/// #             self.set_piece(6, i, Some(Piece::BlackPawn));
/// #         }
///     }
///
///     pub fn hash(&self) -> u64 {
///         self.zobrist.into()
///     }
/// }
///
/// let mut board = ChessBoard::new();
/// board.initialize();
///
/// let initial_hash = board.hash();
///
/// assert_ne!(initial_hash, 0);
///
/// board.set_piece(1, 0, None);
/// let hash_after_move = board.hash();
/// assert_ne!(initial_hash, hash_after_move);
///
/// // Confirm that resetting the board results in the same hash as the initial board
/// board.set_piece(1, 0, Some(Piece::WhitePawn)); // Restore the white pawn
/// let hash_after_reset = board.hash();
/// assert_eq!(initial_hash, hash_after_reset);
/// ```
#[derive(Default, Clone, Copy, Debug)]
pub struct ZobristHashSet<E> {
    hash: u64,
    _data: std::marker::PhantomData<E>,
    #[cfg(all(debug_assertions, feature = "check_set_behavior"))]
    checker: Option<CopiableHash<E>>,
}

impl<E> ZobristHashSet<E> {
    pub fn empty() -> Self {
        Self {
            hash: 0,
            _data: std::marker::PhantomData,
            #[cfg(all(debug_assertions, feature = "check_set_behavior"))]
            checker: Some(CopiableHash::empty()),
        }
    }
}

impl<E> From<u64> for ZobristHashSet<E> {
    fn from(hash: u64) -> Self {
        Self {
            hash,
            _data: std::marker::PhantomData,
            #[cfg(all(debug_assertions, feature = "check_set_behavior"))]
            checker: None,
        }
    }
}

impl<E> From<ZobristHashSet<E>> for u64 {
    fn from(hash: ZobristHashSet<E>) -> u64 {
        hash.hash
    }
}

#[cfg(not(all(debug_assertions, feature = "check_set_behavior")))]
impl<E: Hash + Clone> ZobristHashSet<E> {
    pub fn add(&mut self, key: &E) {
        add_remove_impl(self, key);
    }

    pub fn remove(&mut self, key: &E) {
        add_remove_impl(self, key);
    }
}

#[cfg(all(debug_assertions, feature = "check_set_behavior"))]
impl<E: Hash + Eq + Clone> ZobristHashSet<E> {
    pub fn add(&mut self, key: &E) {
        assert!(self
            .checker
            .as_mut()
            .map(|x| x.insert(key.clone()))
            .unwrap_or(true));
        add_remove_impl(self, key);
    }

    pub fn remove(&mut self, key: &E) {
        assert!(self.checker.as_mut().map(|x| x.remove(key)).unwrap_or(true));
        add_remove_impl(self, key);
    }
}

fn add_remove_impl<E: Hash>(zobrist_hash: &mut ZobristHashSet<E>, key: &E) {
    let mut hasher = FxHasher::default();
    key.hash(&mut hasher);
    zobrist_hash.hash ^= hasher.finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_hash() {
        let mut hash = ZobristHashSet::empty();
        let key = 42;
        hash.add(&key);

        assert_ne!(hash.hash, 0);
        hash.remove(&key);
        assert_eq!(hash.hash, 0);
    }

    #[test]
    fn test_zobrist_hash2() {
        let mut hash = ZobristHashSet::empty();
        let key = (1, 42);
        hash.add(&key);
        let hv = hash.hash;
        let key = (2, 42);
        hash.add(&key);
        assert_ne!(hash.hash, hv);
    }

    #[test]
    fn test_zobrist_hash3() {
        let mut hash1 = ZobristHashSet::empty();
        let key = (1, 42);
        hash1.add(&key);
        let mut hash2 = ZobristHashSet::empty();
        let key = (2, 42);
        hash2.add(&key);
        assert_ne!(hash1.hash, hash2.hash);
    }

    #[test]
    #[should_panic]
    #[cfg(all(debug_assertions, feature = "check_set_behavior"))]
    fn test_zobrist_hash_double_add_debug() {
        let mut hash = ZobristHashSet::empty();
        let key = 42;
        hash.add(&key);
        hash.add(&key);
    }

    #[test]
    #[should_panic]
    #[cfg(all(debug_assertions, feature = "check_set_behavior"))]
    fn test_zobrist_hash_empty_remove_debug() {
        let mut hash = ZobristHashSet::empty();
        let key = 42;
        hash.remove(&key);
    }
}
