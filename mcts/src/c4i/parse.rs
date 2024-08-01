use std::io::stdin;
use std::time::Duration;
use thiserror::Error;

use crate::types::{bitboard::BitBoard, player::Player};

/// Handles the parsing of user IO

/// A user command
pub struct Command(CommandType);

pub enum EngineOption {
    Turn(Player),
}

/// The command type that the user provided the engine with
pub enum CommandType {
    C4i,
    Exit,
    GoInfinite,
    Isready,
    Startpos,
    Stop,

    GoTime(Duration),
    SetOption(EngineOption),

    CustomPosition(BitBoard, BitBoard), // Red, Yellow
}

impl Command {
    pub fn get_type(self) -> CommandType {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum CommandParseError {
    #[error("unable to process user input correctly")]
    IOError,
    #[error("additional expected argument was not found")]
    ExpectedArgument,
    #[error("invalid command: {0}")]
    UnknownCommand(String),
    #[error("invalid argument: {0}")]
    UnknownArgument(String),
}

/// A blocking operation to get the latest user command
pub fn get_command() -> Result<Command, CommandParseError> {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .map_err(|_| CommandParseError::IOError)?;
    let mut parts = buf.split_whitespace();
    let first = parts.next().ok_or(CommandParseError::ExpectedArgument)?;
    let command = match first {
        "c4i" => Command(CommandType::C4i),
        "exit" => Command(CommandType::Exit),
        "stop" => Command(CommandType::Stop),
        "go" => {
            let second = parts.next();
            if second.is_none() {
                Command(CommandType::GoInfinite)
            } else {
                match second.ok_or(CommandParseError::ExpectedArgument)? {
                    "time" => {
                        let micros = parts
                            .next()
                            .ok_or(CommandParseError::ExpectedArgument)?
                            .parse::<u64>()
                            .map_err(|_| CommandParseError::IOError)?;
                        let duration = Duration::from_micros(micros);
                        Command(CommandType::GoTime(duration))
                    }
                    unknown => {
                        return Err(CommandParseError::UnknownArgument(String::from(unknown)))
                    }
                }
            }
        }
        "isready" => Command(CommandType::Isready),
        "position" => {
            let second = parts.next().ok_or(CommandParseError::ExpectedArgument)?;
            match second {
                "startpos" => Command(CommandType::Startpos),
                "custom" => {
                    let third = parts
                        .next()
                        .ok_or(CommandParseError::ExpectedArgument)?
                        .parse::<u64>()
                        .map_err(|_| CommandParseError::IOError)?;
                    let fourth = parts
                        .next()
                        .ok_or(CommandParseError::ExpectedArgument)?
                        .parse::<u64>()
                        .map_err(|_| CommandParseError::IOError)?;
                    Command(CommandType::CustomPosition(
                        BitBoard(third),
                        BitBoard(fourth),
                    ))
                }
                unknown => return Err(CommandParseError::UnknownArgument(String::from(unknown))),
            }
        }
        "setoption" => {
            let second = parts.next().ok_or(CommandParseError::ExpectedArgument)?;
            match second {
                "turn" => {
                    let third = parts.next().ok_or(CommandParseError::ExpectedArgument)?;
                    match third {
                        "red" => Command(CommandType::SetOption(EngineOption::Turn(Player::Red))),
                        "yellow" => {
                            Command(CommandType::SetOption(EngineOption::Turn(Player::Yellow)))
                        }
                        unknown => {
                            return Err(CommandParseError::UnknownArgument(String::from(unknown)))
                        }
                    }
                }
                unknown => return Err(CommandParseError::UnknownArgument(String::from(unknown))),
            }
        }
        unknown => return Err(CommandParseError::UnknownCommand(String::from(unknown))),
    };
    Ok(command)
}
