use crate::bitboard::*;
use colored::Colorize;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Player {
    #[default]
    Yellow,
    Red,
}

impl Player {
    // pub fn signum(&self) -> f32 {
    //     match self {
    //         Player::Red => -1., // Red is the minimizing player
    //         Player::Yellow => 1., // Yellow is the maximizing player
    //     }
    // }
}

impl std::ops::Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

/// A Connect 4 move. Represents the placement of a single stone at a certain column.
#[derive(Default, Debug, Clone, Copy)]
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

#[derive(PartialEq, Default, Debug)]
pub struct Board {
    yellow: BitBoard,
    red: BitBoard,
    pub turn: Player,
}

impl Board {
    // pub fn new(red: BitBoard, yellow: BitBoard, turn: Player) -> Self {
    //     Self { red, yellow, turn }
    // }

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
        // println!("Playing mv {mv:?}");
        let colour = match self.turn {
            Player::Red => &mut self.red,
            Player::Yellow => &mut self.yellow
        };
        *colour |= mv.bitboard();
        self.turn = !self.turn;
    }
    
    pub fn from_column(&self, column: usize) -> Move {
        let all = self.red | self.yellow;
        let bb = ((all.shl(8) | BitBoard(0xff)) ^ all).column(column);
        if bb.is_empty() {
            panic!("Column is full");
        }
        Move(bb)
    }

    // pub fn play(&mut self, mv: Move) {
    //     let mut bb = self.red | self.yellow;
    //     bb |= (bb.shl(8) | BitBoard(0xff)) & BitBoard::A.shl(mv.0);
    //     match self.turn {
    //         Player::Red => self.red = bb ^ self.yellow,
    //         Player::Yellow => self.yellow = bb ^ self.red,
    //     };
    //     self.turn = !self.turn;
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

    // pub fn unplay(&mut self, mv: Move) {
    //     self.turn = !self.turn;
    //     let mut bb = self.red | self.yellow;
    //     bb ^= (bb.shr(8) ^ bb).column(mv.0);
    //     match self.turn {
    //         Player::Red => self.red &= bb,
    //         Player::Yellow => self.yellow &= bb,
    //     }
    // }
}

// macro_rules! impl_colour {
//     ($($c:ident::$b:ident::$m:ident),*) => {
//         $(impl Board {
//             pub fn $c(self) -> BitBoard {
//                 self.$c
//             }

//             pub fn $b(&self) -> &BitBoard {
//                 &self.$c
//             }

//             pub fn $m(&mut self) -> &mut BitBoard {
//                 &mut self.$c
//             }
//         })*
//     };
// }

// impl_colour!(red::red_borrow::red_mut, yellow::yellow_borrow::yellow_mut);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("+---+---+---+---+---+---+---+---+\n");
        for row in (0..8).rev() {
            let mut row_string = String::from("|");
            for col in 0..8 {
                let idx = 8 * row + col;
                let colour;
                if !(BitBoard::idx(idx) & self.red).is_empty() {
                    colour = "⬤".red();
                } else if !(BitBoard::idx(idx) & self.yellow).is_empty() {
                    colour = "⬤".yellow();
                } else {
                    colour = " ".bold(); // remains unchanged
                }
                row_string.push_str(format!(" {colour} |").as_str());
            }
            s.push_str(&row_string);
            s.push_str("\n+---+---+---+---+---+---+---+---+\n");
        }
        s.push_str("  1   2   3   4   5   6   7   8\n");
        s.push_str(format!("Red: {:?}, Yellow: {:?}", self.red, self.yellow).as_str());
        write!(f, "{s}")
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