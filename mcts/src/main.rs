// mod bitboard;
// // mod board;
// mod types;
// mod montecarlo;

// use types::board::*;
// use montecarlo::mcts;

// use std::io::stdin;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut buf = String::new(); // buffer
//     println!("Choose a colour: Red or Yellow. Yellow starts first.");
//     stdin().read_line(&mut buf)?;
//     let us = match buf.chars().next() {
//         Some('r' | 'R') => {
//             // red
//             println!("Ok you will be red. The computer will start first.");
//             Player::Red
//         }
//         _ => {
//             println!("Ok you will be yellow. You will start first.");
//             Player::Yellow
//         }
//     };

//     let mut board = Board::default();
//     while !board.game_over() {
//         if us == board.turn {
//             println!("Choose a column between 1 and 8");
//             buf.clear();
//             stdin().read_line(&mut buf)?;
//             let mut col = buf
//                 .chars()
//                 .next()
//                 .expect("No column provided")
//                 .to_digit(10)
//                 .expect("Please enter number between 1 and 8");
//             if col < 1 || col > 8 {
//                 panic!("Please enter a number between 1 and 8");
//             }
//             col -= 1; // 0..8 (excl.)
//             let mv = board.from_column(col as usize);
//             println!("{mv:?}");
//             board.play(mv);
//         } else {
//             let (root, board_new) = mcts(board);
//             board = board_new;
//             board.play(root.best());
//             root.log_scores();
//             // println!("{board}");
//         }
//     }
//     // println!("{board}");
//     if let Some(colour) = board.winner() {
//         println!("The winner is {colour:?}!");
//     };
//     Ok(())
// }

mod c4i;
mod types;
mod montecarlo;

fn main() {
    let _ = c4i::init();
}