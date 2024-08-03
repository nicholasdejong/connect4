use std::{error::Error, io::Write, time::Duration};

use parse::{get_command, EngineOption};
use search::SearchHandler;

use types::board::Board;
use parse::CommandType as CT;

mod parse;
mod search;

fn send_message(message: &str) {
    println!("{message}");
    std::io::stdout().flush().unwrap();
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let mut search_handler = SearchHandler::new();
    let mut board = Board::default();

    loop {
        let command;
        match get_command() {
            Ok(c) => {command = c}
            Err(e) => {
                println!("warn {e}");
                continue;
            }
        }

        match command.get_type() {
            CT::C4i => {
                send_message("id name Connect4 0.0.1");
                send_message("id author Nicholas de Jong");
                send_message("");
                send_message("option name turn type string default yellow");
                send_message("c4iok")
            }
            CT::Isready => send_message("readyok"),
            CT::Exit => break,
            CT::GoInfinite => {
                search_handler.search(board.clone(), Duration::MAX);
            }
            CT::Startpos => board = Board::default(),
            CT::Stop => {
                search_handler.stop_search();
            }

            CT::SetOption(option) => match option {
                EngineOption::Turn(p) => board.turn = p,
            },
            CT::GoTime(time) => {
                search_handler.search(board.clone(), time);
            }

            CT::CustomPosition(red, yellow) => {
                board.red = red;
                board.yellow = yellow;
            }
        }
        // let mv = search_handler.get_mv();
        // if let Some(mv) = mv {
        //     send_message(format!("bestmove {mv:?}").as_str());
        // }
    }
    Ok(())
}
