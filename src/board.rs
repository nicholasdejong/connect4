use crate::bitboard::*;
use colored::Colorize;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Player {
    #[default]
    Yellow,
    Red,
}

impl Player {
    pub fn signum(&self) -> f32 {
        match self {
            Player::Red => -1.,
            Player::Yellow => 1.,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Board {
    yellow: BitBoard,
    red: BitBoard,
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

    pub fn moves(&self) -> Vec<usize> {
        let all = self.red | self.yellow;
        let bb = all ^ (all.shl(8) | BitBoard(0xff)); // 1st rank
        let mut move_list = Vec::with_capacity(8);
        for i in 0..8 {
            if !(BitBoard::A.shl(i) & bb).is_empty() {
                move_list.push(i);
            }
        }
        move_list
    }

    pub fn game_over(&self) -> bool {
        self.winner().is_some() || (self.red | self.yellow).is_full()
    }

    pub fn play(&mut self, col: usize) {
        let mut bb = self.red | self.yellow;
        bb |= (bb.shl(8) | BitBoard(0xff)) & BitBoard::A.shl(col);
        match self.turn {
            Player::Red => self.red = bb ^ self.yellow,
            Player::Yellow => self.yellow = bb ^ self.red,
        };
        self.turn = !self.turn;
    }

    pub fn unplay(&mut self, col: usize) {
        self.turn = !self.turn;
        let mut bb = self.red | self.yellow;
        bb ^= (bb.shr(8) ^ bb) & BitBoard::A.shl(col);
        match self.turn {
            Player::Red => self.red &= bb,
            Player::Yellow => self.yellow &= bb,
        }
    }
}

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
        s.push_str("  1   2   3   4   5   6   7   8");
        write!(f, "{s}")
    }
}
