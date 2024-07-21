mod bitboard;
mod board;
mod montecarlo;

use board::*;
use montecarlo::negamax;
use rand::thread_rng;

use std::io::stdin;
use std::sync::mpsc;
use std::thread;

// My computer can handle about 25_000_000 random playouts incl. multithreading.
const DEPTH: usize = 5; // The depth is 4 per thread

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new(); // buffer
    println!("Choose a colour: Red or Yellow. Yellow starts first.");
    stdin().read_line(&mut buf)?;
    let us = match buf.chars().next() {
        Some('r' | 'R') => {
            // red
            println!("Ok you will be red. The computer will start first.");
            Player::Red
        }
        _ => {
            println!("Ok you will be yellow. You will start first.");
            Player::Yellow
        }
    };

    let mut board = Board::default();
    while !board.game_over() {
        if us == board.turn {
            println!("Choose a column between 1 and 8");
            buf.clear();
            stdin().read_line(&mut buf)?;
            let mut col = buf
                .chars()
                .next()
                .expect("No column provided")
                .to_digit(10)
                .expect("Please enter number between 1 and 8");
            if col < 1 || col > 8 {
                panic!("Please enter a number between 1 and 8");
            }
            col -= 1; // 0..8 (excl.)
            board.play(col as usize);
        } else {
            let depth = DEPTH - 1;
            let mut receivers = Vec::with_capacity(8);
            for mv in board.moves() {
                let (tx, rx) = mpsc::channel();
                receivers.push(rx);
                thread::spawn(move || {
                    let mut b = board.clone();
                    b.play(mv);
                    let mut rng = thread_rng();
                    let score = -negamax(b, depth - 1, &mut rng).0;

                    tx.send((score, mv)).unwrap();
                });
            }
            let mut moves: Vec<(f32, usize)> =
                receivers.iter().map(|rx| rx.recv().expect("???")).collect();
            moves.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            let best = moves[0];
            println!("Score: {}", best.0);
            board.play(best.1);
            moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let mut last = 0;
            for mv in moves {
                if (mv.1 as isize - last) > 1 {
                    print!("   ");
                } else {
                    print!("{: >4}", (mv.0 * 100.) as isize); // TODO: improve score display
                }
                last = mv.1 as isize;
            }
            println!();
            println!("{board}");
        }
    }
    println!("{:?}", board.winner());
    Ok(())
}
