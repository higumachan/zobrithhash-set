use zobristhash::ZobristHashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

const BOARD_SIZE: usize = 8;

#[derive(Debug)]
struct ChessBoard {
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    zobrist: ZobristHashSet<(usize, usize, Piece)>,
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            zobrist: ZobristHashSet::empty(),
        }
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        if let Some(old_piece) = self.board[x][y] {
            self.zobrist.remove(&(x, y, old_piece));
        }
        if let Some(new_piece) = piece {
            self.zobrist.add(&(x, y, new_piece));
        }
        self.board[x][y] = piece;
    }

    pub fn initialize(&mut self) {
        // Placement of white pieces
        self.set_piece(0, 0, Some(Piece::WhiteRook));
        self.set_piece(0, 1, Some(Piece::WhiteKnight));
        self.set_piece(0, 2, Some(Piece::WhiteBishop));
        self.set_piece(0, 3, Some(Piece::WhiteQueen));
        self.set_piece(0, 4, Some(Piece::WhiteKing));
        self.set_piece(0, 5, Some(Piece::WhiteBishop));
        self.set_piece(0, 6, Some(Piece::WhiteKnight));
        self.set_piece(0, 7, Some(Piece::WhiteRook));
        for i in 0..BOARD_SIZE {
            self.set_piece(1, i, Some(Piece::WhitePawn));
        }

        // Placement of black pieces
        self.set_piece(7, 0, Some(Piece::BlackRook));
        self.set_piece(7, 1, Some(Piece::BlackKnight));
        self.set_piece(7, 2, Some(Piece::BlackBishop));
        self.set_piece(7, 3, Some(Piece::BlackQueen));
        self.set_piece(7, 4, Some(Piece::BlackKing));
        self.set_piece(7, 5, Some(Piece::BlackBishop));
        self.set_piece(7, 6, Some(Piece::BlackKnight));
        self.set_piece(7, 7, Some(Piece::BlackRook));
        for i in 0..BOARD_SIZE {
            self.set_piece(6, i, Some(Piece::BlackPawn));
        }
    }

    pub fn hash(&self) -> u64 {
        self.zobrist.into()
    }
}

fn main() {
    let mut board = ChessBoard::new();
    board.initialize();

    let initial_hash = board.hash();

    assert_ne!(initial_hash, 0);

    board.set_piece(1, 0, None);
    let hash_after_move = board.hash();
    assert_ne!(initial_hash, hash_after_move);

    // Confirm that resetting the board results in the same hash as the initial board
    board.set_piece(1, 0, Some(Piece::WhitePawn)); // Restore the white pawn
    let hash_after_reset = board.hash();
    assert_eq!(initial_hash, hash_after_reset);
}
