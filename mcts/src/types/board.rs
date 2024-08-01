use crate::types::{bitboard::*, player::Player, r#move::Move};
use rand::{rngs::ThreadRng, Rng};

#[derive(PartialEq, Default, Debug, Clone)]
pub struct Board {
    pub yellow: BitBoard,
    pub red: BitBoard,
    pub turn: Player,
}

impl Board {
    pub fn winner(&self) -> Option<Player> {
        if (horizontal(self.yellow)
            | vertical(self.yellow)
            | a1_h8(self.yellow)
            | h1_a8(self.yellow))
        .len()
            > 0
        {
            Some(Player::Yellow)
        } else if (horizontal(self.red) | vertical(self.red) | a1_h8(self.red) | h1_a8(self.red))
            .len()
            > 0
        {
            Some(Player::Red)
        } else {
            None
        }
    }

    /// Returns a BitBoard representing all the legal moves
    pub fn moves(&self) -> BitBoard {
        let all = self.red | self.yellow;
        let bb = all ^ (all.shl(8) | BitBoard(0xff)); // 1st rank
        bb
    }

    pub fn game_over(&self) -> bool {
        self.winner().is_some() || (self.red | self.yellow).is_full()
    }

    pub fn play(&mut self, mv: Move) {
        let colour = match self.turn {
            Player::Red => &mut self.red,
            Player::Yellow => &mut self.yellow
        };
        *colour |= mv.bitboard();
        self.turn = !self.turn;
    }
    
    // pub fn from_column(&self, column: usize) -> Move {
    //     let all = self.red | self.yellow;
    //     let bb = ((all.shl(8) | BitBoard(0xff)) ^ all).column(column);
    //     if bb.is_empty() {
    //         panic!("Column is full");
    //     }
    //     Move::new(bb)
    // }

    pub fn unplay(&mut self, mv: Move) {
        // println!("Undoing mv {mv:?}");
        self.turn = !self.turn;
        let colour = match self.turn {
            Player::Red => &mut self.red,
            Player::Yellow => &mut self.yellow
        };
        *colour ^= mv.bitboard();
    }

}


/// Return a random move from the given move list
pub fn random_move(bb: &BitBoard, rng: &mut ThreadRng) -> Move {
    let moves: Vec<Move> = bb.into();
    moves[rng.gen_range(0..moves.len())]
}

pub fn choose_move(bb: &BitBoard, idx: usize) -> Move {
    let moves: Vec<Move> = bb.into();
    moves[idx]
}

impl From<&BitBoard> for Vec<Move> {
    fn from(bb: &BitBoard) -> Self {
        let mut bb = bb.clone();
        let mut moves: Self = Vec::with_capacity(8);
        loop {
            moves.push(Move::new(BitBoard(bb.0 & 0u64.wrapping_sub(bb.0))));
            bb &= bb - 1;
            if bb.is_empty() { break };
        };
        moves
    }
}