use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::{Board, Player};

const N: usize = 750; // Number of random playouts performed at each root node

// Get a (very rough) estimate of a position by finding the expected score (Win% - Lose%)
fn random_playout(mut rng: &mut ThreadRng, b: Board, count: usize) -> f32 {
    let mut sum = 0.;

    for _ in 0..count {
        let mut b = b.clone();
        while !b.game_over() {
            let moves = b.moves();
            let mv = moves.choose(&mut rng).unwrap(); // If not over, there will always be a move.
            b.play(*mv);
        }
        let result = match b.winner() {
            Some(Player::Yellow) => 1.,
            Some(Player::Red) => -1.,
            None => 0.
        };
        sum += result;
    }

    sum / count as f32
}

pub fn negamax(mut b: Board, depth: usize, mut rng: &mut ThreadRng) -> (f32, usize) {
    // Node is a root node
    if depth == 0 {
        // Commence the random playouts
        // Negate the score if it is Red's turn
        return (random_playout(&mut rng, b, N) * b.turn.signum(), 8);
    }

    let mut best = (f32::NEG_INFINITY, 8); // 8 is not a move

    for mv in b.moves() {
        b.play(mv);
        let score = -negamax(b, depth - 1, &mut rng).0;
        b.unplay(mv);
        if score > best.0 {
            best = (score, mv);
        }
    }

    best
}