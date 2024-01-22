use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GameError {
    UserMoveError(String),
    NotationDecoderError(String),
    StateError(String),

    // hijacked a bit to include ending a game by stalemate or checkmate
    End(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::UserMoveError(err) => write!(f, "User move error: {}", err),
            GameError::NotationDecoderError(err) => write!(f, "Notation decoder error: {}", err),
            GameError::StateError(err) => write!(f, "State error: {}", err),
            GameError::End(err) => write!(f, "End: {}", err),
        }
    }
}

impl Error for GameError {}
