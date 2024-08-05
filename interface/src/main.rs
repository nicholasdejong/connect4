use std::{path::Path, time::Duration};

use engine::Engine;
use game::GameHandler;

mod engine;
mod game;

fn main() {
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).expect("unable to read line 1");
    // let string1 = std::mem::take(&mut buf);
    // std::io::stdin().read_line(&mut buf).expect("unable to read line 2");
    // let string2 = std::mem::take(&mut buf);
    // let (string1, string2) = (string1.trim(), string2.trim());
    let (string1, string2) = ("engines/v0_0_1", "target/release/mcts");
    let (path1, path2) = (Path::new(&string1), Path::new(&string2));

    let (mut engine1, mut engine2) = (Engine::from_path(&path1), Engine::from_path(&path2));
    assert!(engine1.is_c4i());
    assert!(engine2.is_c4i());
    let games = 10;
    let time = Duration::from_millis(100);
    let mut handler = GameHandler::new(engine1, engine2, 1500.0, 1500.0, "Engine 1", "Engine 2");
    let session = handler.play_many(games, time);
    println!("______FINAL RESULTS______");
    println!("{session:?}");
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).expect("unable to read path");
    // let string = std::mem::take(&mut buf);
    // let string = string.trim();
    // let path = Path::new(&string);
    // let mut engine = Engine::from_path(&path);
    // // assert!(engine.is_c4i());
    // println!("Is c4i: {}", engine.is_c4i());
}