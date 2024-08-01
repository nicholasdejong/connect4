use crate::types::bitboard::BitBoard;

/// A Connect 4 move. Represents the placement of a single stone at a certain column.
#[derive(Default, Clone, Copy)]
pub struct Move(BitBoard);

impl Move {
    /// Create a new move from a bitboard
    pub fn new(bb: BitBoard) -> Self {
        Self(bb)
    }

    /// Retrieve the move's bitboard
    pub fn bitboard(self) -> BitBoard {
        self.0
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // println!("{:?}",self.bitboard());
        for col in 0..BitBoard::COLUMNS {
            if !self.bitboard().column(col).is_empty() {
                return write!(f, "{col}");
            }
        };

        write!(f, "None")
    }
}