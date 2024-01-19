use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GameError {
    UserMoveError(String),
    NotationDecoderError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::UserMoveError(err) => write!(f, "User move error: {}", err),
            GameError::NotationDecoderError(err) => write!(f, "Notation decoder error: {}", err),
        }
    }
}

impl Error for GameError {}
