use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::{rngs::ThreadRng, thread_rng};

// use types::Tree;
// use types::node::{Link, Node};
use types::outcome::Outcome;
use types::player::Player;
use types::tree::Tree;
use {
    types::board::{random_move, Board},
    types::r#move::Move,
};

// #[derive(Debug)]
// pub struct Root {
//     children: Vec<Link>,
// }

// impl Root {
//     // Find the best move of the root
//     pub fn best(&self) -> Move {
//         let mut best = f32::NEG_INFINITY;
//         let mut best_mv = None;
//         for child_ref in &self.children {
//             let child = child_ref.borrow();
//             if child.expected_score() > best {
//                 best = child.expected_score();
//                 best_mv = Some(child.mv);
//             }
//         }
//         best_mv.unwrap()
//     }
// }

// impl From<&Board> for Root {
//     fn from(board: &Board) -> Self {
//         let moves: Vec<Move> = (&board.moves()).into();
//         Root {
//             children: moves
//                 .into_iter()
//                 .map(|mv| Rc::new(RefCell::new(mv.into())))
//                 .collect(),
//         }
//     }
// }

// struct Bandit; // An algorithm for choosing a node to investigate.
// impl Bandit {
//     fn select(zipper: Zipper, rounds: usize) -> Zipper {
//         // for child in zipper. {
//         //     if child.borrow().visited == 0 {
//         //         return child.clone();
//         //     }
//         // }
//         let mut best = f32::NEG_INFINITY;
//         let mut best_child = None;
//         for child_ref in children {
//             let child = child_ref.borrow();
//             // Upper Confidence Bound
//             assert!(child.expected_score() >= -1. && child.expected_score() <= 1.);
//             let policy = -child.expected_score()
//                 + 2. * (2. * (rounds as f32).ln() / child.visited as f32).sqrt();
//             if policy > best {
//                 best = policy;
//                 best_child = Some(child_ref);
//             }
//         }
//         best_child.unwrap().clone()
//     }
// }

// /// Selects a child node for expansion
// fn select(root: &Root, board: &mut Board, rounds: usize) -> Link {
//     let child = Bandit::select(&root.children, rounds);
//     fn select(parent: Link, mut board: &mut Board, rounds: usize) -> Link {
//         let mut node = parent.borrow_mut();
//         node.visited += 1;
//         if node.children.len() < board.moves().len() as usize || board.game_over() {
//             return parent.clone();
//         }
//         let child = Bandit::select(&node.children, rounds);
//         board.play(child.borrow().mv);
//         select(child, &mut board, rounds)
//     }
//     board.play(child.borrow().mv);
//     select(child, board, rounds)
// }

// /// Expands the selected node
// fn expand(parent: Link, board: &mut Board) -> Link {
//     if board.game_over() {
//         return parent.clone();
//     }
//     let mut node = parent.borrow_mut();
//     // Random move expansion may result in move redundancy and move exclusion, both of which can make the AI oblivious to certain moves or threats.
//     // Therefore, expansion is deterministic and includes all possible moves
//     let mv = choose_move(&board.moves(), node.children.len());
//     board.play(mv);
//     let child = Node {
//         visited: 1,
//         mv,
//         parent: Some(Link::downgrade(&parent)),
//         children: Vec::new(),
//         ..Default::default()
//     };
//     let child_link = Rc::new(RefCell::new(child));
//     node.children.push(child_link.clone());
//     child_link
// }

/// Play random moves until an outcome is achieved
fn simulate(mut board: &mut Board, mut rng: &mut ThreadRng) -> Outcome {
    // The outcome is in the eyes of the player to move.
    let colour = board.turn; // Find out who should benefit from the outcome
    fn traverse_board(board: &mut Board, colour: Player, mut rng: &mut ThreadRng) -> Outcome {
        if board.game_over() {
            if let Some(winner) = board.winner() {
                if winner == colour {
                    return Outcome::Won;
                } else {
                    return Outcome::Lost;
                }
            } else {
                return Outcome::Drawn;
            }
        }
        // Play a move if it wins immediately
        // For the moment, this check seems to improve playing strength
        // If it were less taxing, the results would be much better
        // TODO: Optimize
        // let bb = board.moves();
        // let moves: Vec<Move> = (&bb).into();
        // for mv in moves {
        //     board.play(mv);
        //     if board.game_over() {
        //         let outcome = if let Some(winner) = board.winner() {
        //             if winner == colour {
        //                 Outcome::Won
        //             } else {
        //                 Outcome::Lost
        //             }
        //         } else {
        //             Outcome::Drawn
        //         };
        //         board.unplay(mv);
        //         return outcome;
        //     } else {
        //         board.unplay(mv);
        //     }
        // }
        // Otherwise, play a random move
        let mv = random_move(&board.moves(), &mut rng);
        board.play(mv);
        let outcome = traverse_board(board, colour, &mut rng);
        board.unplay(mv);
        outcome
    }
    traverse_board(&mut board, colour, &mut rng)
}

// /// Communicate the result with all the parent nodes
// fn backpropagate(link: Link, mut outcome: Outcome, board: &mut Board) {
//     let mut current = Link::downgrade(&link);
//     loop {
//         // Note: This unwrap assumes that the parent will never be dropped before the child
//         // When dropping the List, the nodes are dropped from youngest to oldest, ie the child before the parent
//         let cloned = current.clone().upgrade().unwrap();
//         let mut node = cloned.borrow_mut();
//         board.unplay(node.mv);
//         node.score.update(&outcome);
//         let parent = node.parent.clone();
//         outcome = -outcome; // A win for the one colour is a loss for the other
//         if parent.is_none() {
//             break;
//         } else {
//             current = parent.unwrap();
//         }
//     }
// }

/// Performs the Monte Carlo Tree Search.
pub fn mcts(
    board: &mut Board,
    think_time: Duration,
    should_stop: Arc<AtomicBool>,
    searching: Arc<AtomicBool>,
) -> Move {
    let now = Instant::now();
    let mut tree = Tree::new();
    let mut rng = thread_rng();
    let mut zipper = tree.head();
    let mut rounds = 0;
    loop {
        // println!("{:?}", &board);
        rounds += 1;
        // let selected = zipper.select(board, rounds);
        // let expanded = selected.expand(board);
        let expanded = zipper.select(board, rounds as f32).expand(board);
        let outcome = simulate(board, &mut rng);
        zipper = expanded.backpropagate(board, outcome);
        // println!("{}", zipper.has_parent());
        // println!("{zipper:?}");
        // if rounds > 15 {
        //     tree = zipper.into_tree();
        //     break;
        // }
        if rounds % 1024 == 0 {
            // Check whether to stop every 1024 rounds
            if now.elapsed() > think_time || should_stop.load(Ordering::Acquire) {
                searching.as_ref().store(false, Ordering::Release);
                tree = zipper.into_tree();
                break;
            }
        }
    }
    // tree.log();
    tree.flip();
    // tree.log();
    let mv = tree.best();
    println!("info rounds {rounds}");
    println!("bestmove {mv:?}");
    mv
}
