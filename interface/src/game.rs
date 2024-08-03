use std::time::Duration;
use types::{board::Board, player::Player};

use crate::engine::Engine;

struct Elo(f32);

struct EnginePlayer {
    engine: Engine,
    elo: Elo,
    id: String
}

pub struct GameHandler {
    player1: EnginePlayer,
    player2: EnginePlayer,
}

#[derive(Default, Debug)]
pub struct Session {
    games_played: usize,
    /// Results ordered by (p1 wins, draws, p2 wins)
    results: (usize, usize, usize)
}

impl std::ops::Add for Session {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            games_played: self.games_played + rhs.games_played,
            results: (self.results.0 + rhs.results.0, self.results.1 + rhs.results.1, self.results.2 + rhs.results.2)
        }
    }
}


impl GameHandler {
    pub fn new(engine1: Engine, engine2: Engine, elo1: f32, elo2: f32, id1: &str, id2: &str) -> Self {
        Self {
            player1: EnginePlayer { engine: engine1, elo: Elo(elo1), id: String::from(id1) },
            player2: EnginePlayer { engine: engine2, elo: Elo(elo2), id: String::from(id2) }
        }
    }

    pub fn play(&mut self, time_per_move: Duration, player1: Player) -> Session {
        let mut board = Board::default();
        let mut session = Session::default();
        self.player1.engine.startpos();
        self.player2.engine.startpos();
        while !board.game_over() {
            // println!("mv");
            let engine = if board.turn == player1 { &mut self.player1.engine } else { &mut self.player2.engine };
            if !board.is_empty() {
                // Synchronize the board state across engines
                (*engine).set_position(board.turn, board.red, board.yellow)
            }
            let col = (*engine).get_best(time_per_move);
            let mv = board.from_column(col);
            board.play(mv);
        }
        if let Some(colour) = board.winner() {
            if colour == player1 {
                session.results.0 += 1;
            } else {
                session.results.2 += 1;
            }
        } else {
            session.results.1 += 1;
        }
        session.games_played += 1;
        session
    }

    pub fn play_many(&mut self, count: usize, time_per_move: Duration) -> Session {
        // Each player must have the same amount of tries at each colour
        assert!(count % 2 == 0);
        let mut turn = Player::Yellow;
        let mut session = Session::default();
        for i in 0..count {
            println!("Playing game {}", i + 1);
            let result = self.play(time_per_move, turn);
            println!("Result: {:?}", result.results);
            session = session + result;
            turn = !turn; // Very important. Having the first move is a huge advantage, so each player must have equal chances.
        }
        session
    }
}